cmake_minimum_required(VERSION 3.30...3.31)

project(bungee VERSION ${BUNGEE_VERSION} LANGUAGES C)

# C standard for our FFI layer
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Bungee C library target
add_library(bungee_c STATIC
  bungee/bungee_c.h
  bungee/bungee_c.c)

target_include_directories(bungee_c PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR})

# Link against math library for cos/sin functions
target_link_libraries(bungee_c PRIVATE m)

# Installation
install(FILES 
  bungee/bungee_c.h
  DESTINATION ${CMAKE_INSTALL_PREFIX}/include/bungee)

install(TARGETS bungee_c
  ARCHIVE DESTINATION ${CMAKE_INSTALL_PREFIX}/library
  LIBRARY DESTINATION ${CMAKE_INSTALL_PREFIX}/library
  RUNTIME DESTINATION ${CMAKE_INSTALL_PREFIX}/runtime)
