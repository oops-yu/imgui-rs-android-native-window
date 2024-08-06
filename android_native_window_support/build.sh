 
mkdir build 2>/dev/null  || (echo "creating new dir" && rm -rf build && mkdir build)

cmake   -G "Unix Makefiles" \
        -DCMAKE_MAKE_PROGRAM="$ANDROID_NDK_HOME/prebuilt/linux-x86_64/bin/make" \
        -DCMAKE_TOOLCHAIN_FILE="$ANDROID_NDK_HOME/build/cmake/android.toolchain.cmake" \
        -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
        -DCMAKE_ANDROID_ARCH_ABI="arm64-v8a" \
        -DCMAKE_ANDROID_NDK="$ANDROID_NDK_HOME" \
        -DANDROID_ABI="arm64-v8a" \
        -DANDROID_NDK="$ANDROID_NDK_HOME" \
        -DANDROID_PLATFORM="android-24" \
        -DANDROID_STL="c++_static"\
        -DCMAKE_SYSTEM_NAME=Android \
        -DCMAKE_BUILD_TYPE="Release" \
        -B build\
        -DCMAKE_INSTALL_PREFIX=./ \
        .

cmake --build build
cmake --install build
