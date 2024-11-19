pub struct MergeConfig {
    pub container_img: std::path::PathBuf,
    pub secret_img: std::path::PathBuf,
    pub output_img: std::path::PathBuf,
    pub merge_bits: u8,
}

impl MergeConfig {
    pub fn new(
        container_img: std::path::PathBuf,
        secret_img: std::path::PathBuf,
        output_img: std::path::PathBuf,
        merge_bits: u8,
    ) -> MergeConfig {
        MergeConfig {
            container_img,
            secret_img,
            output_img,
            merge_bits,
        }
    }
}

pub struct UnmergeConfig {
    pub merged_img: std::path::PathBuf,
    pub output_img: std::path::PathBuf,
    pub merge_bits: u8,
}

impl UnmergeConfig {
    pub fn new(
        merged_img: std::path::PathBuf,
        output_img: std::path::PathBuf,
        merge_bits: u8,
    ) -> UnmergeConfig {
        UnmergeConfig {
            merged_img,
            output_img,
            merge_bits,
        }
    }
}

fn merge_pixels(
    cover_pixel: image::Rgb<u8>,
    secret_pixel: image::Rgb<u8>,
    merge_bits: u8,
) -> image::Rgb<u8> {
    let mut merged_pixel = cover_pixel;
    for i in 0..3 {
        let cover_channel = cover_pixel[i];
        let secret_channel = secret_pixel[i];
        let merged_channel =
            (cover_channel & !(0xFF >> merge_bits)) | (secret_channel >> merge_bits);
        merged_pixel[i] = merged_channel;
    }
    merged_pixel
}

fn unmerge_pixels(merged_pixel: image::Rgb<u8>, merge_bits: u8) -> image::Rgb<u8> {
    let mut secret_pixel = image::Rgb([0, 0, 0]);
    for i in 0..3 {
        let merged_channel = merged_pixel[i];
        let secret_channel = merged_channel << merge_bits;
        secret_pixel[i] = secret_channel;
    }
    secret_pixel
}

pub fn merge_images(config: &MergeConfig) -> Result<(), Box<dyn std::error::Error>> {
    let cover_img = image::open(&config.container_img)?;
    let secret_img = image::open(&config.secret_img)?;

    let cover_img = cover_img.to_rgb8();
    let secret_img = secret_img.to_rgb8();

    let (width, height) = cover_img.dimensions();
    let secret_img = image::imageops::resize(
        &secret_img,
        width,
        height,
        image::imageops::FilterType::Nearest,
    );

    let mut merged_img = image::RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let cover_pixel = cover_img.get_pixel(x, y);
            let secret_pixel = secret_img.get_pixel(x, y);
            let merged_pixel = merge_pixels(*cover_pixel, *secret_pixel, config.merge_bits);
            merged_img.put_pixel(x, y, merged_pixel);
        }
    }

    merged_img.save(&config.output_img)?;

    Ok(())
}

pub fn unmerge_images(config: &UnmergeConfig) -> Result<(), Box<dyn std::error::Error>> {
    let merged_img = image::open(&config.merged_img)?;
    let merged_img = merged_img.to_rgb8();
    let (width, height) = merged_img.dimensions();

    let mut secret_img = image::RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let merged_pixel = merged_img.get_pixel(x, y);
            let secret_pixel = unmerge_pixels(*merged_pixel, config.merge_bits);
            secret_img.put_pixel(x, y, secret_pixel);
        }
    }

    secret_img.save(&config.output_img)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;
    use image::Rgb;
    use std::path::Path;
    use std::path::PathBuf;
    use testdir::testdir;

    fn create_merge_test_config(test_dir: &Path) -> MergeConfig {
        MergeConfig {
            container_img: PathBuf::from("examples/container.jpg"),
            secret_img: PathBuf::from("examples/secret.jpg"),
            output_img: test_dir.join("output.png"),
            merge_bits: 4,
        }
    }

    fn create_unmerge_test_config(test_dir: &Path) -> UnmergeConfig {
        UnmergeConfig {
            merged_img: PathBuf::from("examples/merged.png"),
            output_img: test_dir.join("unmerged.jpg"),
            merge_bits: 4,
        }
    }

    #[test]
    fn merge_pixels_merges_nibble() {
        let cover = Rgb([240, 240, 240]); // 11110000
        let secret = Rgb([15, 15, 15]); // 00001111
        let merged = merge_pixels(cover, secret, 4);
        assert_eq!(merged, Rgb([240, 240, 240])); // Should keep high 4 bits of cover and ignore low 4 bits
    }

    #[test]
    fn merge_pixels_merges_zeroed_pixels() {
        let cover = Rgb([0, 0, 0]);
        let secret = Rgb([0, 0, 0]);
        let merged = merge_pixels(cover, secret, 4);
        assert_eq!(merged, Rgb([0, 0, 0]));
    }

    #[test]
    fn merge_pixels_merges_pixels_with_all_bits_set() {
        let cover = Rgb([255, 255, 255]);
        let secret = Rgb([255, 255, 255]);
        let merged = merge_pixels(cover, secret, 4);
        assert_eq!(merged, Rgb([255, 255, 255]));
    }

    #[test]
    fn merge_pixels_merges_pixels_with_mixed_channels() {
        let cover = Rgb([255, 128, 64]);
        let secret = Rgb([15, 15, 15]);
        let merged = merge_pixels(cover, secret, 4);
        assert_eq!(merged, Rgb([240, 128, 64]));
    }

    #[test]
    fn merge_pixels_merges_pixels_using_different_bitmasks() {
        let cover = Rgb([240, 240, 240]); // 11110000
        let secret = Rgb([15, 15, 15]); // 00001111

        // Test all valid bit masks (0-7)
        assert_eq!(merge_pixels(cover, secret, 0), Rgb([15, 15, 15])); // 0 bit:  00001111
        assert_eq!(merge_pixels(cover, secret, 1), Rgb([135, 135, 135])); // 1 bit:  10000111
        assert_eq!(merge_pixels(cover, secret, 2), Rgb([195, 195, 195])); // 2 bits: 11000011
        assert_eq!(merge_pixels(cover, secret, 3), Rgb([225, 225, 225])); // 3 bits: 11100001
        assert_eq!(merge_pixels(cover, secret, 4), Rgb([240, 240, 240])); // 4 bits: 11110000
        assert_eq!(merge_pixels(cover, secret, 5), Rgb([240, 240, 240])); // 5 bits: 11110000
        assert_eq!(merge_pixels(cover, secret, 6), Rgb([240, 240, 240])); // 6 bits: 11001111
        assert_eq!(merge_pixels(cover, secret, 7), Rgb([240, 240, 240])); // 7 bits: 10001111
    }

    #[test]
    fn unmerge_pixels_merges_nibble() {
        let merged = Rgb([255, 255, 255]); // 11111111
        let secret = unmerge_pixels(merged, 4);
        assert_eq!(secret, Rgb([240, 240, 240])); // 11110000
    }

    #[test]
    fn unmerge_pixels_merges_zeroed_pixels() {
        let merged = Rgb([0, 0, 0]);
        let secret = unmerge_pixels(merged, 4);
        assert_eq!(secret, Rgb([0, 0, 0]));
    }

    #[test]
    fn unmerge_pixels_merges_pixels_with_mixed_channels() {
        let merged = Rgb([240, 128, 64]); // 11110000, 10000000, 01000000
        let secret = unmerge_pixels(merged, 4);
        assert_eq!(secret, Rgb([0, 0, 0]));
    }

    #[test]
    fn unmerge_pixels_merges_pixels_using_different_bitmasks() {
        let merged = Rgb([15, 15, 15]); // 00001111

        // Test all valid bit shifts
        assert_eq!(unmerge_pixels(merged, 0), Rgb([15, 15, 15])); // 0 bit:  00001111
        assert_eq!(unmerge_pixels(merged, 1), Rgb([30, 30, 30])); // 1 bit:  00011110
        assert_eq!(unmerge_pixels(merged, 2), Rgb([60, 60, 60])); // 2 bits: 00111100
        assert_eq!(unmerge_pixels(merged, 3), Rgb([120, 120, 120])); // 3 bits: 01111000
        assert_eq!(unmerge_pixels(merged, 4), Rgb([240, 240, 240])); // 4 bits: 11110000
        assert_eq!(unmerge_pixels(merged, 5), Rgb([224, 224, 224])); // 5 bits: 11100000
        assert_eq!(unmerge_pixels(merged, 6), Rgb([192, 192, 192])); // 6 bits: 11000000
        assert_eq!(unmerge_pixels(merged, 7), Rgb([128, 128, 128])); // 7 bits: 10000000
    }

    #[test]
    fn merge_images_successfully_merges_two_valid_img_files(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let test_dir = testdir!();
        let config = create_merge_test_config(&test_dir);

        merge_images(&config)?;

        // Verify output file exists
        assert!(config.output_img.exists());

        // Verify output image dimensions
        let output_img = image::open(&config.output_img)?;
        let cover_img = image::open(&config.container_img)?;
        assert_eq!(output_img.dimensions(), cover_img.dimensions());

        Ok(())
    }

    #[test]
    fn merge_images_returns_error_on_nonexistent_container_img() {
        let test_dir = testdir!();
        let mut config = create_merge_test_config(&test_dir);
        config.container_img = PathBuf::from("nonexistent.png");

        assert!(merge_images(&config).is_err());
    }

    #[test]
    fn merge_images_returns_error_on_nonexistent_secret_img() {
        let test_dir = testdir!();
        let mut config = create_merge_test_config(&test_dir);
        config.secret_img = PathBuf::from("nonexistent.png");

        assert!(merge_images(&config).is_err());
    }

    #[test]
    fn merge_images_sucessfully_merges_imgs_with_different_sizes(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let test_dir = testdir!();
        let config = MergeConfig {
            container_img: PathBuf::from("examples/saruman.webp"),
            secret_img: PathBuf::from("examples/secret.jpg"),
            output_img: test_dir.join("output.png"),
            merge_bits: 4,
        };

        merge_images(&config)?;

        // Get original dimensions
        let cover_img = image::open(&config.container_img)?;
        let secret_img = image::open(&config.secret_img)?;
        let output_img = image::open(&config.output_img)?;

        // Output should match cover image dimensions, not secret image
        assert_eq!(output_img.dimensions(), cover_img.dimensions());
        assert_ne!(output_img.dimensions(), secret_img.dimensions());

        Ok(())
    }

    #[test]
    fn unmerge_images_successfully_decodes_img() -> Result<(), Box<dyn std::error::Error>> {
        let test_dir = testdir!();
        let config = create_unmerge_test_config(&test_dir);

        unmerge_images(&config)?;

        // Verify output file exists
        assert!(config.output_img.exists());

        // Verify output image dimensions
        let merged_img = image::open(&config.merged_img)?;
        let output_img = image::open(&config.output_img)?;
        assert_eq!(output_img.dimensions(), merged_img.dimensions());

        Ok(())
    }

    #[test]
    fn unmerge_images_returns_error_on_nonexistent_merged_img() {
        let test_dir = testdir!();
        let mut config = create_unmerge_test_config(&test_dir);
        config.merged_img = PathBuf::from("nonexistent.png");

        assert!(unmerge_images(&config).is_err());
    }

    #[test]
    fn unmerge_images_returns_error_on_nonexistent_output_img() {
        let test_dir = testdir!();
        let mut config = create_unmerge_test_config(&test_dir);
        config.output_img = PathBuf::from("/nonexistent/directory/secret.png");

        assert!(unmerge_images(&config).is_err());
    }

    #[test]
    fn unmerge_images_preserved_img_dimensions() -> Result<(), Box<dyn std::error::Error>> {
        let test_dir = testdir!();
        let config = create_unmerge_test_config(&test_dir);

        unmerge_images(&config)?;

        let merged_img = image::open(&config.merged_img)?;
        let output_img = image::open(&config.output_img)?;

        assert_eq!(merged_img.dimensions(), output_img.dimensions());

        Ok(())
    }
}
