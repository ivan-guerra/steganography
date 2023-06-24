#ifndef STEGANOGRAPHY_UTIL_HPP_
#define STEGANOGRAPHY_UTIL_HPP_

#include <string>

namespace steganography {

enum class RetCode {
    kSuccess,
    kInvalidFileFormat,
    kFileNotFound,
    kInvalidDimensions,
};

RetCode Merge(const std::string& cover, const std::string& secret,
              const std::string& outfile);

RetCode Unmerge(const std::string& secret, const std::string& outfile);

}  // namespace steganography

#endif
