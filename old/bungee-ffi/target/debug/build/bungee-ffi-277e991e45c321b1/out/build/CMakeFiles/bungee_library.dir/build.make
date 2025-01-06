# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.31

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /opt/homebrew/bin/cmake

# The command to remove a file.
RM = /opt/homebrew/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /Users/bubu/Doc/Github/bungee

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build

# Include any dependencies generated for this target.
include CMakeFiles/bungee_library.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/bungee_library.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/bungee_library.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/bungee_library.dir/flags.make

CMakeFiles/bungee_library.dir/codegen:
.PHONY : CMakeFiles/bungee_library.dir/codegen

bungee.framework/Versions/0.0.0/Headers/bungee/Bungee.h: /Users/bubu/Doc/Github/bungee/bungee/Bungee.h
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Copying OS X content bungee.framework/Versions/0.0.0/Headers/bungee/Bungee.h"
	$(CMAKE_COMMAND) -E copy /Users/bubu/Doc/Github/bungee/bungee/Bungee.h bungee.framework/Versions/0.0.0/Headers/bungee/Bungee.h

bungee.framework/Versions/0.0.0/Headers/bungee/CommandLine.h: /Users/bubu/Doc/Github/bungee/bungee/CommandLine.h
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Copying OS X content bungee.framework/Versions/0.0.0/Headers/bungee/CommandLine.h"
	$(CMAKE_COMMAND) -E copy /Users/bubu/Doc/Github/bungee/bungee/CommandLine.h bungee.framework/Versions/0.0.0/Headers/bungee/CommandLine.h

bungee.framework/Versions/0.0.0/Headers/bungee/Push.h: /Users/bubu/Doc/Github/bungee/bungee/Push.h
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Copying OS X content bungee.framework/Versions/0.0.0/Headers/bungee/Push.h"
	$(CMAKE_COMMAND) -E copy /Users/bubu/Doc/Github/bungee/bungee/Push.h bungee.framework/Versions/0.0.0/Headers/bungee/Push.h

bungee.framework/Versions/0.0.0/Headers/bungee/bungee_c.h: /Users/bubu/Doc/Github/bungee/bungee/bungee_c.h
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Copying OS X content bungee.framework/Versions/0.0.0/Headers/bungee/bungee_c.h"
	$(CMAKE_COMMAND) -E copy /Users/bubu/Doc/Github/bungee/bungee/bungee_c.h bungee.framework/Versions/0.0.0/Headers/bungee/bungee_c.h

bungee.framework/Versions/0.0.0/Headers/bungee/version.h: /Users/bubu/Doc/Github/bungee/bungee/version.h
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Copying OS X content bungee.framework/Versions/0.0.0/Headers/bungee/version.h"
	$(CMAKE_COMMAND) -E copy /Users/bubu/Doc/Github/bungee/bungee/version.h bungee.framework/Versions/0.0.0/Headers/bungee/version.h

CMakeFiles/bungee_library.dir/src/Synthesis.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Synthesis.cpp.o: /Users/bubu/Doc/Github/bungee/src/Synthesis.cpp
CMakeFiles/bungee_library.dir/src/Synthesis.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/bungee_library.dir/src/Synthesis.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Synthesis.cpp.o -MF CMakeFiles/bungee_library.dir/src/Synthesis.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Synthesis.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Synthesis.cpp

CMakeFiles/bungee_library.dir/src/Synthesis.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Synthesis.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Synthesis.cpp > CMakeFiles/bungee_library.dir/src/Synthesis.cpp.i

CMakeFiles/bungee_library.dir/src/Synthesis.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Synthesis.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Synthesis.cpp -o CMakeFiles/bungee_library.dir/src/Synthesis.cpp.s

CMakeFiles/bungee_library.dir/src/Basic.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Basic.cpp.o: /Users/bubu/Doc/Github/bungee/src/Basic.cpp
CMakeFiles/bungee_library.dir/src/Basic.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/bungee_library.dir/src/Basic.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Basic.cpp.o -MF CMakeFiles/bungee_library.dir/src/Basic.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Basic.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Basic.cpp

CMakeFiles/bungee_library.dir/src/Basic.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Basic.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Basic.cpp > CMakeFiles/bungee_library.dir/src/Basic.cpp.i

CMakeFiles/bungee_library.dir/src/Basic.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Basic.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Basic.cpp -o CMakeFiles/bungee_library.dir/src/Basic.cpp.s

CMakeFiles/bungee_library.dir/src/Fourier.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Fourier.cpp.o: /Users/bubu/Doc/Github/bungee/src/Fourier.cpp
CMakeFiles/bungee_library.dir/src/Fourier.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Building CXX object CMakeFiles/bungee_library.dir/src/Fourier.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Fourier.cpp.o -MF CMakeFiles/bungee_library.dir/src/Fourier.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Fourier.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Fourier.cpp

CMakeFiles/bungee_library.dir/src/Fourier.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Fourier.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Fourier.cpp > CMakeFiles/bungee_library.dir/src/Fourier.cpp.i

CMakeFiles/bungee_library.dir/src/Fourier.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Fourier.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Fourier.cpp -o CMakeFiles/bungee_library.dir/src/Fourier.cpp.s

CMakeFiles/bungee_library.dir/src/Grain.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Grain.cpp.o: /Users/bubu/Doc/Github/bungee/src/Grain.cpp
CMakeFiles/bungee_library.dir/src/Grain.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_4) "Building CXX object CMakeFiles/bungee_library.dir/src/Grain.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Grain.cpp.o -MF CMakeFiles/bungee_library.dir/src/Grain.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Grain.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Grain.cpp

CMakeFiles/bungee_library.dir/src/Grain.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Grain.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Grain.cpp > CMakeFiles/bungee_library.dir/src/Grain.cpp.i

CMakeFiles/bungee_library.dir/src/Grain.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Grain.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Grain.cpp -o CMakeFiles/bungee_library.dir/src/Grain.cpp.s

CMakeFiles/bungee_library.dir/src/Grains.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Grains.cpp.o: /Users/bubu/Doc/Github/bungee/src/Grains.cpp
CMakeFiles/bungee_library.dir/src/Grains.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_5) "Building CXX object CMakeFiles/bungee_library.dir/src/Grains.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Grains.cpp.o -MF CMakeFiles/bungee_library.dir/src/Grains.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Grains.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Grains.cpp

CMakeFiles/bungee_library.dir/src/Grains.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Grains.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Grains.cpp > CMakeFiles/bungee_library.dir/src/Grains.cpp.i

CMakeFiles/bungee_library.dir/src/Grains.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Grains.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Grains.cpp -o CMakeFiles/bungee_library.dir/src/Grains.cpp.s

CMakeFiles/bungee_library.dir/src/Input.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Input.cpp.o: /Users/bubu/Doc/Github/bungee/src/Input.cpp
CMakeFiles/bungee_library.dir/src/Input.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_6) "Building CXX object CMakeFiles/bungee_library.dir/src/Input.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Input.cpp.o -MF CMakeFiles/bungee_library.dir/src/Input.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Input.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Input.cpp

CMakeFiles/bungee_library.dir/src/Input.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Input.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Input.cpp > CMakeFiles/bungee_library.dir/src/Input.cpp.i

CMakeFiles/bungee_library.dir/src/Input.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Input.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Input.cpp -o CMakeFiles/bungee_library.dir/src/Input.cpp.s

CMakeFiles/bungee_library.dir/src/Output.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Output.cpp.o: /Users/bubu/Doc/Github/bungee/src/Output.cpp
CMakeFiles/bungee_library.dir/src/Output.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_7) "Building CXX object CMakeFiles/bungee_library.dir/src/Output.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Output.cpp.o -MF CMakeFiles/bungee_library.dir/src/Output.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Output.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Output.cpp

CMakeFiles/bungee_library.dir/src/Output.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Output.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Output.cpp > CMakeFiles/bungee_library.dir/src/Output.cpp.i

CMakeFiles/bungee_library.dir/src/Output.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Output.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Output.cpp -o CMakeFiles/bungee_library.dir/src/Output.cpp.s

CMakeFiles/bungee_library.dir/src/Partials.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Partials.cpp.o: /Users/bubu/Doc/Github/bungee/src/Partials.cpp
CMakeFiles/bungee_library.dir/src/Partials.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_8) "Building CXX object CMakeFiles/bungee_library.dir/src/Partials.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Partials.cpp.o -MF CMakeFiles/bungee_library.dir/src/Partials.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Partials.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Partials.cpp

CMakeFiles/bungee_library.dir/src/Partials.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Partials.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Partials.cpp > CMakeFiles/bungee_library.dir/src/Partials.cpp.i

CMakeFiles/bungee_library.dir/src/Partials.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Partials.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Partials.cpp -o CMakeFiles/bungee_library.dir/src/Partials.cpp.s

CMakeFiles/bungee_library.dir/src/Stretch.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Stretch.cpp.o: /Users/bubu/Doc/Github/bungee/src/Stretch.cpp
CMakeFiles/bungee_library.dir/src/Stretch.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_9) "Building CXX object CMakeFiles/bungee_library.dir/src/Stretch.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Stretch.cpp.o -MF CMakeFiles/bungee_library.dir/src/Stretch.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Stretch.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Stretch.cpp

CMakeFiles/bungee_library.dir/src/Stretch.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Stretch.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Stretch.cpp > CMakeFiles/bungee_library.dir/src/Stretch.cpp.i

CMakeFiles/bungee_library.dir/src/Stretch.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Stretch.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Stretch.cpp -o CMakeFiles/bungee_library.dir/src/Stretch.cpp.s

CMakeFiles/bungee_library.dir/src/Timing.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Timing.cpp.o: /Users/bubu/Doc/Github/bungee/src/Timing.cpp
CMakeFiles/bungee_library.dir/src/Timing.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_10) "Building CXX object CMakeFiles/bungee_library.dir/src/Timing.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Timing.cpp.o -MF CMakeFiles/bungee_library.dir/src/Timing.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Timing.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Timing.cpp

CMakeFiles/bungee_library.dir/src/Timing.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Timing.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Timing.cpp > CMakeFiles/bungee_library.dir/src/Timing.cpp.i

CMakeFiles/bungee_library.dir/src/Timing.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Timing.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Timing.cpp -o CMakeFiles/bungee_library.dir/src/Timing.cpp.s

CMakeFiles/bungee_library.dir/src/Window.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Window.cpp.o: /Users/bubu/Doc/Github/bungee/src/Window.cpp
CMakeFiles/bungee_library.dir/src/Window.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_11) "Building CXX object CMakeFiles/bungee_library.dir/src/Window.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Window.cpp.o -MF CMakeFiles/bungee_library.dir/src/Window.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Window.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Window.cpp

CMakeFiles/bungee_library.dir/src/Window.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Window.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Window.cpp > CMakeFiles/bungee_library.dir/src/Window.cpp.i

CMakeFiles/bungee_library.dir/src/Window.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Window.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Window.cpp -o CMakeFiles/bungee_library.dir/src/Window.cpp.s

CMakeFiles/bungee_library.dir/src/Assert.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/src/Assert.cpp.o: /Users/bubu/Doc/Github/bungee/src/Assert.cpp
CMakeFiles/bungee_library.dir/src/Assert.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_12) "Building CXX object CMakeFiles/bungee_library.dir/src/Assert.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/src/Assert.cpp.o -MF CMakeFiles/bungee_library.dir/src/Assert.cpp.o.d -o CMakeFiles/bungee_library.dir/src/Assert.cpp.o -c /Users/bubu/Doc/Github/bungee/src/Assert.cpp

CMakeFiles/bungee_library.dir/src/Assert.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/src/Assert.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/src/Assert.cpp > CMakeFiles/bungee_library.dir/src/Assert.cpp.i

CMakeFiles/bungee_library.dir/src/Assert.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/src/Assert.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/src/Assert.cpp -o CMakeFiles/bungee_library.dir/src/Assert.cpp.s

CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.o: CMakeFiles/bungee_library.dir/flags.make
CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.o: /Users/bubu/Doc/Github/bungee/bungee/bungee_c.cpp
CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.o: CMakeFiles/bungee_library.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_13) "Building CXX object CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.o -MF CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.o.d -o CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.o -c /Users/bubu/Doc/Github/bungee/bungee/bungee_c.cpp

CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/bubu/Doc/Github/bungee/bungee/bungee_c.cpp > CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.i

CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/bubu/Doc/Github/bungee/bungee/bungee_c.cpp -o CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.s

# Object files for target bungee_library
bungee_library_OBJECTS = \
"CMakeFiles/bungee_library.dir/src/Synthesis.cpp.o" \
"CMakeFiles/bungee_library.dir/src/Basic.cpp.o" \
"CMakeFiles/bungee_library.dir/src/Fourier.cpp.o" \
"CMakeFiles/bungee_library.dir/src/Grain.cpp.o" \
"CMakeFiles/bungee_library.dir/src/Grains.cpp.o" \
"CMakeFiles/bungee_library.dir/src/Input.cpp.o" \
"CMakeFiles/bungee_library.dir/src/Output.cpp.o" \
"CMakeFiles/bungee_library.dir/src/Partials.cpp.o" \
"CMakeFiles/bungee_library.dir/src/Stretch.cpp.o" \
"CMakeFiles/bungee_library.dir/src/Timing.cpp.o" \
"CMakeFiles/bungee_library.dir/src/Window.cpp.o" \
"CMakeFiles/bungee_library.dir/src/Assert.cpp.o" \
"CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.o"

# External object files for target bungee_library
bungee_library_EXTERNAL_OBJECTS =

bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Synthesis.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Basic.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Fourier.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Grain.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Grains.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Input.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Output.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Partials.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Stretch.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Timing.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Window.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/src/Assert.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/bungee/bungee_c.cpp.o
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/build.make
bungee.framework/Versions/0.0.0/bungee: CMakeFiles/bungee_library.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_14) "Linking CXX static library bungee.framework/bungee"
	$(CMAKE_COMMAND) -P CMakeFiles/bungee_library.dir/cmake_clean_target.cmake
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/bungee_library.dir/link.txt --verbose=$(VERBOSE)

bungee.framework/bungee: bungee.framework/Versions/0.0.0/bungee
	@$(CMAKE_COMMAND) -E touch_nocreate bungee.framework/bungee

# Rule to build all files generated by this target.
CMakeFiles/bungee_library.dir/build: bungee.framework/bungee
CMakeFiles/bungee_library.dir/build: bungee.framework/Versions/0.0.0/Headers/bungee/Bungee.h
CMakeFiles/bungee_library.dir/build: bungee.framework/Versions/0.0.0/Headers/bungee/CommandLine.h
CMakeFiles/bungee_library.dir/build: bungee.framework/Versions/0.0.0/Headers/bungee/Push.h
CMakeFiles/bungee_library.dir/build: bungee.framework/Versions/0.0.0/Headers/bungee/bungee_c.h
CMakeFiles/bungee_library.dir/build: bungee.framework/Versions/0.0.0/Headers/bungee/version.h
.PHONY : CMakeFiles/bungee_library.dir/build

CMakeFiles/bungee_library.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/bungee_library.dir/cmake_clean.cmake
.PHONY : CMakeFiles/bungee_library.dir/clean

CMakeFiles/bungee_library.dir/depend:
	cd /Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/bubu/Doc/Github/bungee /Users/bubu/Doc/Github/bungee /Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build /Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build /Users/bubu/Doc/Github/bungee/bungee-ffi/target/debug/build/bungee-ffi-277e991e45c321b1/out/build/CMakeFiles/bungee_library.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/bungee_library.dir/depend
