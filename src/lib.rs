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
