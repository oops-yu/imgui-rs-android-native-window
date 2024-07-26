#ifndef __ANATIVE_WINDOW_CREATOR_H
#define __ANATIVE_WINDOW_CREATOR_H
#include <android/native_window.h>
#include <android/log.h>
#include <dlfcn.h>
#include <sys/system_properties.h>

#include <cstddef>
#include <unordered_map>
#include <string>
#include <vector>
#include <stdio.h>
namespace android
{
    struct DisplayInfo
    {
        int32_t orientation;
        int32_t width;
        int32_t height;
    };
    extern "C"
    {

        DisplayInfo get_display_info();

        ANativeWindow *create_native_window(const char *, int32_t, int32_t, bool);

        void destroy_native_window(ANativeWindow *);

        DisplayInfo greeting();
    }
} // namespace android

#endif