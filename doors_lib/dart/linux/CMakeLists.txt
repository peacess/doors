# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

# Project-level configuration.
set(PROJECT_NAME "idl")
project(${PROJECT_NAME} LANGUAGES C)

set(idl_bundled_libraries
#  $<TARGET_FILE:idl>
  "${CMAKE_CURRENT_SOURCE_DIR}/libffi_rpc.so"
  PARENT_SCOPE
)