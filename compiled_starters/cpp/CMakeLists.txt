cmake_minimum_required(VERSION 3.13)

project(codecrafters-tokenizer)

set(CMAKE_CXX_STANDARD 23)

file(GLOB_RECURSE SOURCE_FILES src/*.cpp src/*.hpp)

add_executable(tokenizer ${SOURCE_FILES})

find_package(nlohmann_json CONFIG REQUIRED)
target_link_libraries(tokenizer PRIVATE nlohmann_json::nlohmann_json)

# PCRE2 can run GPT-2's split pattern, which std::regex can't.
find_package(pcre2 CONFIG REQUIRED)
target_link_libraries(tokenizer PRIVATE PCRE2::8BIT)
target_compile_definitions(tokenizer PRIVATE PCRE2_CODE_UNIT_WIDTH=8)
