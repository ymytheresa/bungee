cmake_minimum_required(VERSION 3.25)

set(CMAKE_CXX_STANDARD 20)

project(bungee)

file(GLOB BUNGEE_LIBRARY_HEADERS "${CMAKE_CURRENT_SOURCE_DIR}/bungee/*.h")

add_library(bungee_object OBJECT 
  ${BUNGEE_LIBRARY_HEADERS}
  src/Synthesis.cpp
  src/Basic.cpp
  src/Fourier.cpp
  src/Fourier.cpp
  src/Grain.cpp
  src/Grains.cpp
  src/Input.cpp
  src/Output.cpp
  src/Partials.cpp
  src/Stretch.cpp
  src/Timing.cpp
  src/Window.cpp
  src/Assert.cpp)

target_include_directories(bungee_object PUBLIC .)
target_include_directories(bungee_object PRIVATE submodules submodules/eigen)

set(BUNGEE_SELF_TEST 0 CACHE STRING "Enable self tests (0=off, 1=fast, 2=full)")
target_compile_definitions(bungee_object PRIVATE BUNGEE_SELF_TEST=${BUNGEE_SELF_TEST} eigen_assert=BUNGEE_ASSERT1)

target_compile_options(bungee_object PRIVATE $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-fwrapv>)

set_target_properties(bungee_object PROPERTIES
  PUBLIC_HEADER "${BUNGEE_LIBRARY_HEADERS}"
  POSITION_INDEPENDENT_CODE ON)

add_custom_command(
  OUTPUT version_static.cpp never_exists_static
  COMMAND ${CMAKE_COMMAND} -DVERSION_NAMESPACE=Bungee -DGIT_CWD=${CMAKE_CURRENT_SOURCE_DIR} -DVERSION_CPP=version_static.cpp -P ${CMAKE_CURRENT_SOURCE_DIR}/version.cmake)
  
add_library(bungee_static STATIC
  ${BUNGEE_LIBRARY_HEADERS}
  ${CMAKE_CURRENT_BINARY_DIR}/version_static.cpp
  $<TARGET_OBJECTS:bungee_object>)
  
add_custom_command(
  OUTPUT version_shared.cpp never_exists_shared
  COMMAND ${CMAKE_COMMAND} -DVERSION_NAMESPACE=Bungee -DGIT_CWD=${CMAKE_CURRENT_SOURCE_DIR} -DVERSION_CPP=version_shared.cpp -P ${CMAKE_CURRENT_SOURCE_DIR}/version.cmake)
  
add_library(bungee_shared SHARED
  ${BUNGEE_LIBRARY_HEADERS}
  ${CMAKE_CURRENT_BINARY_DIR}/version_shared.cpp
  $<TARGET_OBJECTS:bungee_object>)

add_executable(bungee_executable cmd/main.cpp)
target_link_libraries(bungee_executable PRIVATE bungee_static)
target_include_directories(bungee_executable PRIVATE submodules/cxxopts/include .)

set_target_properties(bungee_static PROPERTIES 
  OUTPUT_NAME bungee
  PUBLIC_HEADER "${BUNGEE_LIBRARY_HEADERS}")
set_target_properties(bungee_shared PROPERTIES 
  OUTPUT_NAME bungee)
set_target_properties(bungee_executable PROPERTIES 
  OUTPUT_NAME bungee)

set(KISSFFT_PKGCONFIG OFF CACHE INTERNAL "" FORCE)
set(KISSFFT_STATIC ON CACHE INTERNAL "" FORCE)
set(KISSFFT_TEST OFF CACHE INTERNAL "" FORCE)
set(KISSFFT_TOOLS OFF CACHE INTERNAL "" FORCE)
add_subdirectory(submodules/kissfft EXCLUDE_FROM_ALL)
target_link_libraries(bungee_static PRIVATE kissfft)
target_link_libraries(bungee_shared PRIVATE kissfft)
target_link_libraries(bungee_executable PRIVATE kissfft)

if(APPLE)
  set(BUNGEE_VERSION 0.0 CACHE STRING "Bungee framework version string")
  set_target_properties(bungee_shared PROPERTIES
    FRAMEWORK TRUE
    MACOSX_FRAMEWORK_IDENTIFIER com.parabolaresearch.bungee
    XCODE_ATTRIBUTE_CODE_SIGN_IDENTITY "iPhone Developer"
    VERSION ${BUNGEE_VERSION}
    SOVERSION ${BUNGEE_VERSION}
    MACOSX_FRAMEWORK_BUNDLE_VERSION ${BUNGEE_VERSION}
    MACOSX_FRAMEWORK_SHORT_VERSION_STRING ${BUNGEE_VERSION}
    )
endif()

install(FILES cmd/main.cpp
  DESTINATION ${CMAKE_INSTALL_PREFIX}/sample)

install(TARGETS bungee_static
  PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_PREFIX}/include
  ARCHIVE DESTINATION ${CMAKE_INSTALL_PREFIX}/${BUNGEE_ABI_NAME}/archive)

install(TARGETS bungee_shared
  LIBRARY DESTINATION ${CMAKE_INSTALL_PREFIX}/${BUNGEE_ABI_NAME}/library
  RUNTIME DESTINATION ${CMAKE_INSTALL_PREFIX}/${BUNGEE_ABI_NAME}/runtime
  FRAMEWORK DESTINATION ${CMAKE_INSTALL_PREFIX}/${BUNGEE_ABI_NAME}/framework)

install(TARGETS bungee_executable
  RUNTIME DESTINATION ${CMAKE_INSTALL_PREFIX}/${BUNGEE_ABI_NAME}/runtime
  BUNDLE DESTINATION ${CMAKE_INSTALL_PREFIX}/${BUNGEE_ABI_NAME}/bundle)
