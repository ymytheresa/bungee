
set(BUNGEE_VERSION 0.0.0 CACHE STRING "Bungee version string")
set(BUNGEE_SELF_TEST 0 CACHE STRING "Enable Bungee self tests (0=off, 1=fast, 2=full)")
option(BUNGEE_BUILD_STATIC_LIBRARIES "Build static, not dynamic, Bungee libraries" OFF)

cmake_minimum_required(VERSION 3.30...3.31)

project(bungee VERSION ${BUNGEE_VERSION})
set(CMAKE_CXX_STANDARD 20)

# Bungee library target: "bungee_library"
file(GLOB BUNGEE_LIBRARY_HEADERS "${CMAKE_CURRENT_SOURCE_DIR}/bungee/*.h")
add_custom_command(OUTPUT version.cpp _ COMMAND ${CMAKE_COMMAND} -DVERSION_NAMESPACE=Bungee -DGIT_CWD=${CMAKE_CURRENT_SOURCE_DIR} -DVERSION_CPP=version.cpp -P ${CMAKE_CURRENT_SOURCE_DIR}/version.cmake)
set(BUNGEE_LIBRARY_SOURCES
  src/Synthesis.cpp
  src/Basic.cpp
  src/Fourier.cpp
  src/Grain.cpp
  src/Grains.cpp
  src/Input.cpp
  src/Output.cpp
  src/Partials.cpp
  src/Stretch.cpp
  src/Timing.cpp
  src/Window.cpp
  src/Assert.cpp
  ${CMAKE_CURRENT_BINARY_DIR}/version.cpp
)
if(BUNGEE_BUILD_STATIC_LIBRARIES)
  add_library(bungee_library STATIC ${BUNGEE_LIBRARY_HEADERS} ${BUNGEE_LIBRARY_SOURCES})
else()
  add_library(bungee_library SHARED ${BUNGEE_LIBRARY_HEADERS} ${BUNGEE_LIBRARY_SOURCES})
endif()
#set_target_properties(bungee_library PROPERTIES 
#  OUTPUT_NAME bungee)
target_include_directories(bungee_library PUBLIC 
  .)
target_include_directories(bungee_library PRIVATE 
  submodules 
  submodules/eigen)
target_compile_definitions(bungee_library PRIVATE
  BUNGEE_SELF_TEST=${BUNGEE_SELF_TEST}
  eigen_assert=BUNGEE_ASSERT1
  EIGEN_DONT_PARALLELIZE=1)
target_compile_options(bungee_library PRIVATE 
  $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-fwrapv>)
set_target_properties(bungee_library PROPERTIES
  OUTPUT_NAME bungee)
target_link_libraries(bungee_library PRIVATE 
  kissfft)

# Bungee command-line executable target: "bungee_executable"
add_executable(bungee_executable 
  cmd/main.cpp)
target_link_libraries(bungee_executable PRIVATE 
  bungee_library)
target_include_directories(bungee_executable PRIVATE 
  submodules/cxxopts/include 
  .)
set_target_properties(bungee_executable PROPERTIES 
  OUTPUT_NAME bungee)

# KissFFT build
set(CMAKE_WARN_DEPRECATED OFF CACHE BOOL "" FORCE)
set(KISSFFT_PKGCONFIG OFF CACHE INTERNAL "" FORCE)
set(KISSFFT_STATIC ON CACHE INTERNAL "" FORCE)
set(KISSFFT_TEST OFF CACHE INTERNAL "" FORCE)
set(KISSFFT_TOOLS OFF CACHE INTERNAL "" FORCE)
add_subdirectory(submodules/kissfft EXCLUDE_FROM_ALL)

# Further optimisation and hygiene for Release mode
include(CheckIPOSupported)
check_ipo_supported(RESULT supported OUTPUT error)
if(supported)
    message(STATUS "IPO / LTO enabled")
    set_property(TARGET bungee_library PROPERTY INTERPROCEDURAL_OPTIMIZATION $<CONFIG:Release>>)
    set_property(TARGET bungee_executable PROPERTY INTERPROCEDURAL_OPTIMIZATION $<CONFIG:Release>)
else()
    message(STATUS "IPO / LTO not supported: <${error}>")
endif()
target_compile_options(bungee_library PRIVATE $<$<CONFIG:Release>:-fno-rtti -fvisibility=hidden>)
add_custom_command(TARGET bungee_library POST_BUILD
  COMMAND $<$<CONFIG:Release>:${CMAKE_STRIP}> 
  ARGS $<IF:$<BOOL:${APPLE}>,-u,-s> $<TARGET_FILE:bungee_library>)
add_custom_command(TARGET bungee_executable POST_BUILD
  COMMAND $<$<CONFIG:Release>:${CMAKE_STRIP}> 
  ARGS $<IF:$<BOOL:${APPLE}>,,-s> $<TARGET_FILE:bungee_executable>)

# Installation
if (APPLE)
  set_target_properties(bungee_library PROPERTIES
    FRAMEWORK TRUE
    XCODE_ATTRIBUTE_CODE_SIGN_IDENTITY "iPhone Developer"
    MACOSX_FRAMEWORK_BUNDLE_VERSION ${BUNGEE_VERSION}
    MACOSX_FRAMEWORK_SHORT_VERSION_STRING ${BUNGEE_VERSION}
    VERSION ${BUNGEE_VERSION}
    )
  set_source_files_properties(
    ${BUNGEE_LIBRARY_HEADERS} 
    PROPERTIES
    MACOSX_PACKAGE_LOCATION Headers/bungee)
else()
  install(FILES 
    ${BUNGEE_LIBRARY_HEADERS}
    DESTINATION ${CMAKE_INSTALL_PREFIX}/include/bungee)
endif()
install(TARGETS bungee_library
  ARCHIVE DESTINATION ${CMAKE_INSTALL_PREFIX}/library
  LIBRARY DESTINATION ${CMAKE_INSTALL_PREFIX}/library
  RUNTIME DESTINATION ${CMAKE_INSTALL_PREFIX}/runtime
  FRAMEWORK DESTINATION ${CMAKE_INSTALL_PREFIX})
install(FILES 
  cmd/main.cpp  
  submodules/cxxopts/include/cxxopts.hpp
  DESTINATION ${CMAKE_INSTALL_PREFIX}/sample)
