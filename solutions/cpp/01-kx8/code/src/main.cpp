#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>

#include <nlohmann/json.hpp>

using json = nlohmann::json;

int main(int argc, char *argv[]) {
    std::cout << std::unitbuf;
    std::cerr << std::unitbuf;

    if (argc < 2) {
        std::cerr << "No command provided.\n";
        return EXIT_FAILURE;
    }

    std::string command = argv[1];
    if (command == "bytes") {
        std::ifstream file(argv[2], std::ios::binary);
        std::vector<unsigned char> data((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());
        std::cout << json(data).dump() << "\n";
    } else {
        std::cerr << "Unknown command " << command << "\n";
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
