set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
cmake_minimum_required(VERSION 3.28)
project(untitled1 C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_FLAGS "-fblocks")

include_directories("./include")
find_package(PkgConfig REQUIRED)
link_directories("/opt/homebrew/lib")


add_executable(spice ./include/main.c ./include/session.c ./include/channel.c ./include/connection.c)

pkg_check_modules(SPICE_CLIENT REQUIRED spice-client-glib-2.0)
include_directories(${SPICE_CLIENT_INCLUDE_DIRS})
add_definitions(${SPICE_CLIENT_CFLAGS_OTHER})
target_link_libraries(spice ${SPICE_CLIENT_LIBRARIES})
