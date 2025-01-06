#pragma once

#include <fstream>
#include <string>
#include <chrono>
#include <cstdarg>
#include <mutex>
#include <thread>

class DebugLog {
public:
    static void init(const char* filename) {
        std::lock_guard<std::mutex> lock(mutex());
        if (!file.is_open()) {
            file.open(filename, std::ios::out | std::ios::trunc);
            start_time = std::chrono::steady_clock::now();
            logf("Debug log initialized [%s]", filename);
        }
    }

    static void log(const char* msg) {
        std::lock_guard<std::mutex> lock(mutex());
        if (file.is_open()) {
            auto now = std::chrono::steady_clock::now();
            auto elapsed = std::chrono::duration_cast<std::chrono::microseconds>(now - start_time);
            file << "[" << elapsed.count() << "µs][" << std::this_thread::get_id() << "] " << msg << std::endl;
            file.flush();
        }
    }

    static void logf(const char* fmt, ...) {
        char buffer[1024];
        va_list args;
        va_start(args, fmt);
        vsnprintf(buffer, sizeof(buffer), fmt, args);
        va_end(args);
        log(buffer);
    }

    static void close() {
        std::lock_guard<std::mutex> lock(mutex());
        if (file.is_open()) {
            log("Debug log closed");
            file.close();
        }
    }

private:
    static std::mutex& mutex() {
        static std::mutex m;
        return m;
    }

    static std::ofstream file;
    static std::chrono::steady_clock::time_point start_time;
};

std::ofstream DebugLog::file;
std::chrono::steady_clock::time_point DebugLog::start_time;

#ifdef DEBUG_FFI
    #define DBG_LOG(msg) DebugLog::log(msg)
    #define DBG_LOGF(...) DebugLog::logf(__VA_ARGS__)
#else
    #define DBG_LOG(msg)
    #define DBG_LOGF(...)
#endif 