#include <cstdlib>
#include <iostream>
#include <string>

void PrintUsage() {
    std::cout << "usage: steganography CMD IN_IMG... OUT_IMG" << std::endl;
    std::cout << "\tCMD" << std::endl;
    std::cout
        << "\t\tOne of either 'merge' or 'unmerge'. The 'merge' command will "
           "merge the\n\t\tsecond IN_IMG argument into the first IN_IMG "
           "argument producing a new\n\t\tOUT_IMG. The 'unmerge' command will "
           "extract an image to OUT_IMG previ-\n\t\tously embedded in the "
           "IN_IMG argument."
        << std::endl;
    std::cout << "\tIN_IMG\n\t\tA JPEG or PNG image." << std::endl;
    std::cout << "\tOUT_IMG\n\t\tA JPEG or PNG image containing the result of "
                 "executing the parameter\n\t\tCMD."
              << std::endl;
    std::cout << "EXAMPLES" << std::endl;
    std::cout << "\tsteganography merge container.png secret.jpg out.png"
              << std::endl;
    std::cout << "\tsteganography unmerge out.png secret.jpg" << std::endl;
    std::cout << "\n\tNOTE: The output of the merge command and input to the "
                 "unmerge command will a-\n\tlways be a PNG."
              << std::endl;
}

void PrintErrAndExit(const std::string& err) {
    std::cerr << "error: " << err << std::endl;
    std::cout << std::endl;
    PrintUsage();
    exit(EXIT_FAILURE);
}

int main(int argc, char** argv) {
    const int kMergeCmdArgCount = 5;
    const int kUnmergeCmdArgCount = 4;
    const std::string kMergeCmd("merge");
    const std::string kUnmergeCmd("unmerge");

    /* did the user give the right number args for a merge/unmerge command? */
    if (argc < std::min(kMergeCmdArgCount, kUnmergeCmdArgCount)) {
        PrintErrAndExit("invalid arg count");
    }

    /* did the user specify a valid command? */
    std::string cmd(argv[1]);
    if ((kMergeCmd != cmd) && (kUnmergeCmd != cmd)) {
        PrintErrAndExit("unknown CMD value");
    } else { /* we have a valid command but do we have the right arg count? */
        if ((kMergeCmd == cmd) && (kMergeCmdArgCount != argc)) {
            PrintErrAndExit("invalid arg count for merge command");
        } else if ((kUnmergeCmd == cmd) && (kUnmergeCmdArgCount != argc)) {
            PrintErrAndExit("invalid arg count for unmerge command");
        }
    }

    /* TODO: Add logic to merge/unmerge image files. */
    return 0;
}
