# Install script for directory: /Users/bubu/Doc/Github/bungee

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set path to fallback-tool for dependency-resolution.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/bungee.framework")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out" TYPE DIRECTORY FILES "/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/bungee.framework" USE_SOURCE_PERMISSIONS)
  if(EXISTS "$ENV{DESTDIR}/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/bungee.framework/Versions/0.0.0/bungee" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/bungee.framework/Versions/0.0.0/bungee")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/bungee.framework/Versions/0.0.0/bungee")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/sample/main.cpp;/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/sample/cxxopts.hpp")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/sample" TYPE FILE FILES
    "/Users/bubu/Doc/Github/bungee/cmd/main.cpp"
    "/Users/bubu/Doc/Github/bungee/submodules/cxxopts/include/cxxopts.hpp"
    )
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
if(CMAKE_INSTALL_LOCAL_ONLY)
  file(WRITE "/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/install_local_manifest.txt"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
endif()
if(CMAKE_INSTALL_COMPONENT)
  if(CMAKE_INSTALL_COMPONENT MATCHES "^[a-zA-Z0-9_.+-]+$")
    set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
  else()
    string(MD5 CMAKE_INST_COMP_HASH "${CMAKE_INSTALL_COMPONENT}")
    set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INST_COMP_HASH}.txt")
    unset(CMAKE_INST_COMP_HASH)
  endif()
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  file(WRITE "/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
endif()
