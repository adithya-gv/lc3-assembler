#include <iostream>
#include <fstream>
#include <string>
#include <vector>

int main(int argc, char *argv[]) {
    // Step 1: Open the target file.
    std::string filename;

    // Check if a file was specified. 
    if (argc != 2) {
        std::cout << "Usage: " << argv[0] << " <filename>" << std::endl;
        return 1;
    }
    filename = argv[1];

    // Check if the file is a valid assembly file. 
    int index = filename.find(".asm");
    if (index == -1) {
        std::cout << "Error: " << filename << " is not a .asm file" << std::endl;
        return 1;
    }
    
    // Prepare values to start reading the file.
    std::string output = "output.txt";
    std::ifstream file(filename);
    std::ofstream out(output);
    if (!file.is_open()) {
        std::cout << "Error: " << filename << " could not be opened" << std::endl;
        return 1;
    }

    // Get assembling data structures ready.
    std::vector<std::string> tokens = {};
    std::vector<std::string> labels = {};
    std::vector<int> addresses = {};
    std::vector<std::string> values = {};

    int pc = 0x00;

    // Step 2: Parse Assembly Code.  
    std::string line;
    int lineCount = 0;
    while (getline(file, line)) {
        pc++;
        lineCount++;

        // First Case: .orig
        if (line.find(".orig") != -1) {

            // Get the address.
            int index = line.find(" ");
            if (index == -1) {
                std::cout << "Error: Invalid expression on line " << lineCount << std::endl;
                return 1;
            }
            std::string address = line.substr(index + 1);
            pc = std::stoi(address, nullptr, 16);

            continue;

        }

        // Second Case: .fill
        if (line.find(".fill") != -1) {

            // Remove .fill from the line.
            int index = line.find(" ");
            if (index == -1) {
                std::cout << "Error: Invalid expression on line " << lineCount << std::endl;
                return 1;
            }

            // Get the label name.
            std::string leftover = line.substr(index + 1);
            index = leftover.find(" ");
            if (index == -1) {
                std::cout << "Error: Invalid expression on line " << lineCount << std::endl;
                return 1;
            }
            std::string label = leftover.substr(0, index);
            labels.push_back(label);
            addresses.push_back(pc - 1);

            // Get the value.
            std::string value = leftover.substr(index + 1);
            values.push_back(value);
            
            continue;
        }

        // Third Case: .end
        if (line.find(".end") != -1) {
            continue;
        }

        // Fourth Case: Actual Instructions

        // Get the instruction name.

        // Split into cases based on the instruction name.

    }

}