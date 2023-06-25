#include <cstdlib>
#include <iostream>
#include <string>

#include "utils/steganography_util.hpp"

void PrintUsage() {
    std::cout << "usage: steganography CMD IN_IMG... OUT_IMG" << std::endl;
    std::cout << "\tCMD" << std::endl;
    std::cout << "\t\tone of 'merge', 'unmerge', or 'help'" << std::endl;
    std::cout << "\tIN_IMG\n\t\ta jpeg or png image" << std::endl;
    std::cout << "\tOUT_IMG\n\t\ta jpeg or png image containing the result of "
                 "executing the parameter\n\t\tCMD"
              << std::endl;
    std::cout << "EXAMPLES" << std::endl;
    std::cout << "\tsteganography merge container.png secret.jpg out.png"
              << std::endl;
    std::cout << "\tsteganography unmerge out.png secret.jpg" << std::endl;
    std::cout << "NOTES" << std::endl;
    std::cout << "\tThe output of the merge command and input to the unmerge "
                 "command must\n\talways be a PNG!"
              << std::endl;
}

void PrintErrAndExit(const std::string& err) {
    std::cerr << "error: " << err << std::endl;
    std::cerr << "try 'steganography help' for more information" << std::endl;
    exit(EXIT_FAILURE);
}

int main(int argc, char** argv) {
    const int kMergeCmdArgCount = 5;
    const int kUnmergeCmdArgCount = 4;
    const std::string kMergeCmd("merge");
    const std::string kUnmergeCmd("unmerge");
    const std::string kHelpCmd("help");

    if (argc < 2) { /* missing the program command arg */
        PrintErrAndExit("missing command");
    }

    /* did the user specify a valid command? */
    std::string cmd(argv[1]);
    if ((kMergeCmd != cmd) && (kUnmergeCmd != cmd) && (kHelpCmd != cmd)) {
        PrintErrAndExit("unknown CMD value");
    } else { /* we have a valid command but do we have the right arg count? */
        if ((kMergeCmd == cmd) && (kMergeCmdArgCount != argc)) {
            PrintErrAndExit("invalid arg count for merge command");
        } else if ((kUnmergeCmd == cmd) && (kUnmergeCmdArgCount != argc)) {
            PrintErrAndExit("invalid arg count for unmerge command");
        }
    }

    /* execute the merge/unmerge command */
    steganography::RetCode rc = steganography::RetCode::kSuccess;
    if (kMergeCmd == cmd) {
        rc = steganography::Merge(argv[2], argv[3], argv[4]);
    } else if (kUnmergeCmd == cmd) {
        rc = steganography::Unmerge(argv[2], argv[3]);
    } else if (kHelpCmd == cmd) {
        PrintUsage();
    }

    /* report errors if there are any */
    switch (rc) {
        case steganography::RetCode::kSuccess:
            break;
        case steganography::RetCode::kInvalidFileFormat:
            PrintErrAndExit("invalid format, only JPEG and PNG are accepted");
            break;
        case steganography::RetCode::kFileNotFound:
            PrintErrAndExit("one or more input files do not exist");
            break;
        case steganography::RetCode::kInvalidDimensions:
            PrintErrAndExit("secret image does not fit inside cover image");
            break;
    }
    return 0;
}
