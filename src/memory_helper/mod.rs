use once_cell::sync::Lazy;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::ptr;
#[allow(dead_code)]
const OP_INIT_KEY: u64 = 0x900;
const OP_READ_MEM: u64 = 0x901;
const OP_WRITE_MEM: u64 = 0x902;
const OP_MODULE_BASE: u64 = 0x903;

#[repr(C)]
struct CopyMemory {
    pid: libc::pid_t,
    addr: u64,
    buffer: *mut libc::c_void,
    size: usize,
    offsets_count: usize,
    offsets: [u64; 10],
}

#[repr(C)]
struct ModuleBase {
    pid: libc::pid_t,
    name: *mut u8,
    base: u64,
}

pub struct GameMem {
    fd: File,
    pid: libc::pid_t,
    buffer: [u8; 1024],
    additional_offset: u64,
    is_additional_offset_negative: bool,
    need_additional_offset: bool,
}

impl GameMem {
    pub fn new() -> Self {
        let file = File::options()
            .read(true)
            .write(true)
            .open("/proc/kmsg")
            .expect("[-] open driver failed");
        GameMem {
            fd: file,
            pid: 0,
            buffer: [0; 1024],
            additional_offset: 0,
            is_additional_offset_negative: true,
            need_additional_offset: false,
        }
    }

    pub fn initialize_with_pid(&mut self, pid: libc::pid_t) {
        self.pid = pid;
    }
    pub fn initialize_with_process_name(&mut self, process_name: &str) {
        loop {
            match get_name_pid(process_name) {
                Ok(pid) => {
                    self.pid = pid;
                    break;
                }
                Err(msg) => {
                    println!("{}", msg);
                    std::thread::sleep(Duration::from_secs(3));
                }
            }
        }
    }
    pub fn read_memory_with_offsets<T: Default>(
        &self,
        addr: u64,
        buffer: *mut T,
        offsets: &[u64],
    ) -> bool {
        let mut cm = CopyMemory {
            pid: self.pid,
            addr,
            buffer: buffer as *mut libc::c_void,
            size: std::mem::size_of::<T>(),
            offsets_count: offsets.len(),
            offsets: [0; 10],
        };

        if cm.offsets_count > 10 {
            return false;
        }

        for (dst, &src) in cm.offsets.iter_mut().zip(offsets.iter()) {
            *dst = src;
        }
        if self.need_additional_offset {
            if self.is_additional_offset_negative {
                cm.offsets[cm.offsets_count - 1] -= self.additional_offset;
            } else {
                cm.offsets[cm.offsets_count - 1] += self.additional_offset;
            }
        }

        let fd = self.fd.as_raw_fd();
        let res = unsafe { libc::ioctl(fd, OP_READ_MEM as _, &cm) };

        if res != 0 {
            return false;
        }

        true
    }
    pub fn read_memory_with_length_and_offsets(
        &self,
        addr: u64,
        buffer: *mut libc::c_void,
        length: usize,
        offsets: &[u64],
    ) -> bool {
        let mut cm = CopyMemory {
            pid: self.pid,
            addr,
            buffer: buffer as *mut libc::c_void,
            size: length,
            offsets_count: offsets.len(),
            offsets: [0; 10],
        };

        if cm.offsets_count > 10 {
            return false;
        }

        for (dst, &src) in cm.offsets.iter_mut().zip(offsets.iter()) {
            *dst = src;
        }
        if self.need_additional_offset {
            if self.is_additional_offset_negative {
                cm.offsets[cm.offsets_count - 1] -= self.additional_offset;
            } else {
                cm.offsets[cm.offsets_count - 1] += self.additional_offset;
            }
        }

        let fd = self.fd.as_raw_fd();
        let res = unsafe { libc::ioctl(fd, OP_READ_MEM as _, &cm) };

        if res != 0 {
            return false;
        }

        true
    }

    pub fn get_module_base(&self, name: &str) -> Option<u64> {
        let c_name = std::ffi::CString::new(name).unwrap();
        let mut mb = ModuleBase {
            pid: self.pid,
            name: c_name.into_raw() as _,
            base: 0,
        };

        let fd = self.fd.as_raw_fd();
        let res = unsafe { libc::ioctl(fd, OP_MODULE_BASE as _, &mut mb) };

        if res != 0 {
            return None;
        }

        Some(mb.base)
    }

    pub fn read_with_offsets<T: Default>(&mut self, addr: u64, offsets: &[u64]) -> T {
        if self.read_memory_with_offsets(addr, self.buffer.as_ptr() as *mut T, offsets) {
            return unsafe { ptr::read(self.buffer.as_ptr() as *const T) };
        }
        Default::default()
    }
    pub fn set_additional_offset(&mut self, offset: u64, negative: bool) {
        self.additional_offset = offset;
        self.is_additional_offset_negative = negative;
        self.need_additional_offset = true;
    }
    pub fn un_set_additional_offset(&mut self) {
        self.need_additional_offset = false;
    }
}
#[allow(unused_imports)]
use std::process::Command;
use std::time::Duration;
fn get_name_pid(name: &str) -> Result<i32, String> {
    // 构造命令字符串
    let cmd = format!("pidof {}", name);

    // 执行命令并获取输出
    let output = Command::new("sh").arg("-c").arg(cmd).output();
    match output {
        Ok(output) => {
            // 检查命令是否成功执行
            if !output.status.success() {
                Err(format!("pidof {name} command exec failed."))
            } else {
                // 将命令输出转换为字符串
                let stdout = String::from_utf8(output.stdout);

                // 尝试从输出中解析 PID
                let pid = stdout.unwrap_or(String::new()).trim().parse::<i32>().ok();
                if let Some(pid) = pid {
                    Ok(pid)
                } else {
                    Err("cannot find process,waiting for opening game".to_string())
                }
            }
        }
        _ => return Err(format!("pidof {name} command exec failed.")),
    }
}

static mut DRIVER: Lazy<GameMem> = Lazy::new(|| GameMem::new());
#[allow(dead_code)]
pub fn get_mem() -> &'static mut GameMem {
    unsafe { &mut *DRIVER }
}
