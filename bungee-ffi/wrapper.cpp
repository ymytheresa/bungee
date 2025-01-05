#include <bungee/Bungee.h>

extern "C" {

// Export the function table for FFI
Bungee_Stretcher_FunctionTable Bungee_Stretcher_getFunctionTable() {
    return Bungee::FunctionTable<Bungee::Basic>::get();
}

} 