use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Merge(MergeArgs),
    Unmerge(UnmergeArgs),
}

#[derive(Args)]
#[command(about = "Merge a secret image into a container image")]
struct MergeArgs {
    #[arg(help = "container image path")]
    container_img: std::path::PathBuf,

    #[arg(help = "secret image path")]
    secret_img: std::path::PathBuf,

    #[arg(help = "output image path")]
    output_img: std::path::PathBuf,

    #[arg(help = "number of texture points")]
    merge_bits: u8,
}

#[derive(Args)]
#[command(about = "Extract a hidden image from a container image")]
struct UnmergeArgs {
    #[arg(help = "container image path")]
    merged_img: std::path::PathBuf,

    #[arg(help = "output image path")]
    output_img: std::path::PathBuf,

    #[arg(help = "number of texture points")]
    merge_bits: u8,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Merge(args) => {
            let config = steg::MergeConfig::new(
                args.container_img.clone(),
                args.secret_img.clone(),
                args.output_img.clone(),
                args.merge_bits,
            );
        }
        Commands::Unmerge(args) => {
            let config = steg::UnmergeConfig::new(
                args.merged_img.clone(),
                args.output_img.clone(),
                args.merge_bits,
            );
        }
    }
}
