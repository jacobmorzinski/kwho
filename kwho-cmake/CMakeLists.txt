cmake_minimum_required(VERSION 2.8.11 FATAL_ERROR)

IF (NOT DEFINED KWHO_DIR_PREFIX)
    set (KWHO_DIR_PREFIX ${CMAKE_SOURCE_DIR})
ENDIF (NOT DEFINED KWHO_DIR_PREFIX)

project(KWHO)
set(CMAKE_MODULE_PATH ${KWHO_DIR_PREFIX}/cmake/Modules)

# This module discovers KRB5_INCLUDE_DIRS and KRB5_LIBRARY_NAMES
find_package(Krb5 REQUIRED krb5)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -Wall")

# include dirs
include_directories(
    ${KRB5_INCLUDE_DIRS}
)

add_subdirectory(${KWHO_DIR_PREFIX}/src src)
