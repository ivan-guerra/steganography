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
