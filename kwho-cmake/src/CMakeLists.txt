#include_directories(${KWHO_SOURCE_DIR}/kwho)
#link_directories(${KWHO_BINARY_DIR}/kwho)

add_executable(kwho kwho.c)
target_link_libraries(kwho ${KRB5_LIBRARY_NAMES})

