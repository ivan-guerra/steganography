#include "utils/steganography_util.hpp"

#include <algorithm>
#include <boost/gil.hpp>
#include <boost/gil/extension/io/jpeg.hpp>
#include <boost/gil/extension/io/png.hpp>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <string>
#include <vector>

namespace steganography {

enum class ImageType {
    kJpeg,
    kPng,
    kUnknown,
};

static ImageType GetImageType(const std::string& filename) {
    /* read the first 8 bytes of the file */
    const int kHeaderSize = 8;
    std::ifstream ifs(filename, std::ifstream::binary);
    if (!ifs.is_open()) {
        return ImageType::kUnknown;
    }
    std::vector<char> buffer(kHeaderSize, 0);
    ifs.read(&buffer[0], static_cast<int64_t>(buffer.size()));

    /* construct an unsigned 64-bit word using the 8 bytes in buffer */
    const int kByteShift = 8;
    uint64_t word = 0;
    for (const char& c : buffer) {
        word = (word << kByteShift) | static_cast<uint8_t>(c);
    }

    /* check if the word matches a known image file type signature */
    const uint64_t kPngSignature = 0x89504E470D0A1A0A;
    const uint64_t kJpegSignature = 0xFFD8000000000000;
    if (word == kPngSignature) {
        return ImageType::kPng;
    } else if ((word & kJpegSignature) == kJpegSignature) {
        return ImageType::kJpeg;
    }
    return ImageType::kUnknown;
}

static boost::gil::rgb8_image_t ReadImage(const std::string& filename,
                                          ImageType type) {
    boost::gil::rgb8_image_t image;
    if (type == ImageType::kJpeg) {
        boost::gil::read_and_convert_image(filename, image,
                                           boost::gil::jpeg_tag{});
    } else {
        boost::gil::read_and_convert_image(filename, image,
                                           boost::gil::png_tag{});
    }
    return image;
}

static void WriteImage(const boost::gil::rgb8_image_t& image,
                       const std::string& filename, ImageType type) {
    if (type == ImageType::kJpeg) {
        boost::gil::write_view(filename, boost::gil::const_view(image),
                               boost::gil::jpeg_tag{});
    } else {
        boost::gil::write_view(filename, boost::gil::const_view(image),
                               boost::gil::png_tag{});
    }
}

static boost::gil::rgb8_pixel_t MergePixels(
    const boost::gil::rgb8_pixel_t& cover_pix,
    const boost::gil::rgb8_pixel_t& secret_pix) {
    const int kHighNibble = 0xF0;
    boost::gil::rgb8_pixel_t merge(0, 0, 0);
    for (int i = 0; i < 3; ++i) {
        merge[i] =
            (cover_pix[i] & kHighNibble) | ((secret_pix[i] & kHighNibble) >> 4);
    }
    return merge;
}

static boost::gil::rgb8_pixel_t UnmergePixels(
    const boost::gil::rgb8_pixel_t& pixel) {
    const int kLowNibble = 0x0F;
    boost::gil::rgb8_pixel_t secret_pix(0, 0, 0);
    for (int i = 0; i < 3; ++i) {
        secret_pix[i] = (pixel[i] & kLowNibble) << 4;
    }
    return secret_pix;
}

static bool HasJpegExtension(const std::string& filename) {
    const std::vector<std::string> kExtensions = {".jpg", ".jpeg", ".JPG",
                                                  ".JPEG"};
    return std::any_of(
        kExtensions.cbegin(), kExtensions.cend(),
        [&filename](const std::string& s) { return filename.ends_with(s); });
}

RetCode Merge(const std::string& cover, const std::string& secret,
              const std::string& outfile) {
    /* verify the input image files exists */
    if (!std::filesystem::exists(cover) || !std::filesystem::exists(secret)) {
        return RetCode::kFileNotFound;
    }

    /* verify the input image files have a valid file type */
    ImageType cover_img_t(GetImageType(cover));
    ImageType secret_img_t(GetImageType(secret));
    if ((cover_img_t == ImageType::kUnknown) ||
        (secret_img_t == ImageType::kUnknown)) {
        return RetCode::kInvalidFileFormat;
    }

    /* load images into GIL image type */
    boost::gil::rgb8_image_t cover_img(ReadImage(cover, cover_img_t));
    boost::gil::rgb8_image_t secret_img(ReadImage(secret, secret_img_t));
    boost::gil::rgb8_image_t output_img = cover_img;

    /* verify secret fits within cover */
    if ((secret_img.height() > cover_img.height()) ||
        (secret_img.width() > cover_img.width())) {
        return RetCode::kInvalidDimensions;
    }

    /* merge the secret image's pixels into the output image */
    const boost::gil::rgb8_pixel_t kBlackPixel(0, 0, 0);
    auto secret_view = boost::gil::const_view(secret_img);
    auto output_view = boost::gil::view(output_img);
    for (int row = 0; row < output_view.height(); ++row) {
        for (int col = 0; col < output_view.width(); ++col) {
            if ((row >= secret_img.height()) || (col >= secret_img.width())) {
                output_view(col, row) =
                    MergePixels(output_view(col, row), kBlackPixel);
            } else {
                output_view(col, row) =
                    MergePixels(output_view(col, row), secret_view(col, row));
            }
        }
    }

    WriteImage(output_img, outfile, ImageType::kPng);

    return RetCode::kSuccess;
}

RetCode Unmerge(const std::string& secret, const std::string& outfile) {
    /* verify the image containing the secret exists */
    if (!std::filesystem::exists(secret)) {
        return RetCode::kFileNotFound;
    }

    /* verify the input image has a valid file type */
    ImageType secret_img_t(GetImageType(secret));
    if (secret_img_t == ImageType::kUnknown) {
        return RetCode::kInvalidFileFormat;
    }

    /* load images into GIL image type */
    boost::gil::rgb8_image_t secret_img(ReadImage(secret, secret_img_t));
    boost::gil::rgb8_image_t output_img = secret_img;

    /* extract the hidden image into the output image */
    auto secret_view = boost::gil::const_view(secret_img);
    auto output_view = boost::gil::view(output_img);
    for (int row = 0; row < output_view.height(); ++row) {
        for (int col = 0; col < output_view.width(); ++col) {
            output_view(col, row) = UnmergePixels(secret_view(col, row));
        }
    }

    if (HasJpegExtension(outfile)) {
        WriteImage(output_img, outfile, ImageType::kJpeg);
    } else {
        WriteImage(output_img, outfile, ImageType::kPng);
    }

    return RetCode::kSuccess;
}

}  // namespace steganography
