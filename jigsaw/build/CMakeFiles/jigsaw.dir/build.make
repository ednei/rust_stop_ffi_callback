# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.22

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
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/es/rust_stop_ffi_callback/jigsaw

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/es/rust_stop_ffi_callback/jigsaw/build

# Include any dependencies generated for this target.
include CMakeFiles/jigsaw.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/jigsaw.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/jigsaw.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/jigsaw.dir/flags.make

CMakeFiles/jigsaw.dir/jigsaw.cpp.o: CMakeFiles/jigsaw.dir/flags.make
CMakeFiles/jigsaw.dir/jigsaw.cpp.o: ../jigsaw.cpp
CMakeFiles/jigsaw.dir/jigsaw.cpp.o: CMakeFiles/jigsaw.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/es/rust_stop_ffi_callback/jigsaw/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/jigsaw.dir/jigsaw.cpp.o"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT CMakeFiles/jigsaw.dir/jigsaw.cpp.o -MF CMakeFiles/jigsaw.dir/jigsaw.cpp.o.d -o CMakeFiles/jigsaw.dir/jigsaw.cpp.o -c /home/es/rust_stop_ffi_callback/jigsaw/jigsaw.cpp

CMakeFiles/jigsaw.dir/jigsaw.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/jigsaw.dir/jigsaw.cpp.i"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/es/rust_stop_ffi_callback/jigsaw/jigsaw.cpp > CMakeFiles/jigsaw.dir/jigsaw.cpp.i

CMakeFiles/jigsaw.dir/jigsaw.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/jigsaw.dir/jigsaw.cpp.s"
	/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/es/rust_stop_ffi_callback/jigsaw/jigsaw.cpp -o CMakeFiles/jigsaw.dir/jigsaw.cpp.s

# Object files for target jigsaw
jigsaw_OBJECTS = \
"CMakeFiles/jigsaw.dir/jigsaw.cpp.o"

# External object files for target jigsaw
jigsaw_EXTERNAL_OBJECTS =

libjigsaw.a: CMakeFiles/jigsaw.dir/jigsaw.cpp.o
libjigsaw.a: CMakeFiles/jigsaw.dir/build.make
libjigsaw.a: CMakeFiles/jigsaw.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/es/rust_stop_ffi_callback/jigsaw/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Linking CXX static library libjigsaw.a"
	$(CMAKE_COMMAND) -P CMakeFiles/jigsaw.dir/cmake_clean_target.cmake
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/jigsaw.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/jigsaw.dir/build: libjigsaw.a
.PHONY : CMakeFiles/jigsaw.dir/build

CMakeFiles/jigsaw.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/jigsaw.dir/cmake_clean.cmake
.PHONY : CMakeFiles/jigsaw.dir/clean

CMakeFiles/jigsaw.dir/depend:
	cd /home/es/rust_stop_ffi_callback/jigsaw/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/es/rust_stop_ffi_callback/jigsaw /home/es/rust_stop_ffi_callback/jigsaw /home/es/rust_stop_ffi_callback/jigsaw/build /home/es/rust_stop_ffi_callback/jigsaw/build /home/es/rust_stop_ffi_callback/jigsaw/build/CMakeFiles/jigsaw.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/jigsaw.dir/depend
