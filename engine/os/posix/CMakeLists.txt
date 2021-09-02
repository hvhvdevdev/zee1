project(Zee1_Posix)

add_library(Zee1_Posix STATIC lib.c)

target_link_libraries(Zee1_Posix PUBLIC SDL2)

install(TARGETS Zee1_Posix)