cmake_minimum_required(VERSION 3.5.0)
project(trap_pad VERSION 0.1.0)

include(CTest)
enable_testing()

# Add the source files
set(SOURCE_FILES src/main.cpp src/gui.cpp src/editor.cpp)

# Include directories for header files
include_directories(${PROJECT_SOURCE_DIR}/src)

# Add executable based on the source files
add_executable(trap_pad ${SOURCE_FILES})

# Link SDL2 and SDL2_ttf libraries
find_package(SDL2 REQUIRED)
find_package(SDL2_ttf REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS} ${SDL2_TTF_INCLUDE_DIRS})
target_link_libraries(trap_pad ${SDL2_LIBRARIES} ${SDL2_TTF_LIBRARIES} GL)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
