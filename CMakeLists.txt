cmake_minimum_required(VERSION 3.25)

set(CMAKE_CXX_STANDARD 20)

add_custom_command(
  OUTPUT version.cpp _
  COMMAND ${CMAKE_COMMAND} -DVERSION_NAMESPACE=Bungee -DGIT_CWD=${CMAKE_CURRENT_SOURCE_DIR} -P ${CMAKE_CURRENT_SOURCE_DIR}/version.cmake)

project(bungee)

file(GLOB BUNGEE_LIBRARY_HEADERS "${CMAKE_CURRENT_SOURCE_DIR}/bungee/*.h")

add_library(bungee_static STATIC
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
  src/Assert.cpp
  ${CMAKE_CURRENT_BINARY_DIR}/version.cpp)

target_include_directories(bungee_static PUBLIC .)
target_include_directories(bungee_static PRIVATE submodules submodules/eigen)

set(BUNGEE_SELF_TEST 0 CACHE STRING "Enable self tests (0=off, 1=fast, 2=full)")
target_compile_definitions(bungee_static PRIVATE BUNGEE_SELF_TEST=${BUNGEE_SELF_TEST} eigen_assert=BUNGEE_ASSERT1)

target_compile_options(bungee_static PRIVATE $<$<NOT:$<CXX_COMPILER_ID:MSVC>>:-fwrapv>)

set_target_properties(bungee_static PROPERTIES
  PUBLIC_HEADER "${BUNGEE_LIBRARY_HEADERS}"
  POSITION_INDEPENDENT_CODE ON)

add_library(bungee_shared SHARED
  ${BUNGEE_LIBRARY_HEADERS})

target_link_libraries(bungee_shared PUBLIC bungee_static)

add_executable(bungee_executable cmd/main.cpp $<TARGET_OBJECTS:bungee_static>)
target_include_directories(bungee_executable PRIVATE submodules/cxxopts/include .)

set_target_properties(bungee_static PROPERTIES OUTPUT_NAME bungee)
set_target_properties(bungee_shared PROPERTIES OUTPUT_NAME bungee)
set_target_properties(bungee_executable PROPERTIES OUTPUT_NAME bungee)

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
    PUBLIC_HEADER "${BUNGEE_LIBRARY_HEADERS}"
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
  LIBRARY DESTIMATION ${CMAKE_INSTALL_PREFIX}/${BUNGEE_ABI_NAME}/library
  RUNTIME DESTIMATION ${CMAKE_INSTALL_PREFIX}/${BUNGEE_ABI_NAME}/runtime
  FRAMEWORK DESTINATION ${CMAKE_INSTALL_PREFIX}/${BUNGEE_ABI_NAME}/framework)

install(TARGETS bungee_executable
  RUNTIME DESTINATION ${CMAKE_INSTALL_PREFIX}/${BUNGEE_ABI_NAME}/runtime
  BUNDLE DESTINATION ${CMAKE_INSTALL_PREFIX}/${BUNGEE_ABI_NAME}/bundle)
