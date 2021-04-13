use evg_bonsai::build_landscape;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "evg-bonsai")]
enum Opt {
    /// Generate an Evergreen YAML configuration from a given bonsai source.
    Build {
        /// File to build evergreen configuration from.
        #[structopt(parse(from_os_str), long = "source-file")]
        source_file: PathBuf,

        /// Directory write generated content to.
        #[structopt(parse(from_os_str), long = "target-dir", default_value = ".")]
        target_dir: PathBuf,

        /// Filename to use for generated output.
        #[structopt(long = "target-filename", default_value = "evergreen.yml")]
        target_filename: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    match opt {
        Opt::Build {
            source_file,
            target_dir,
            target_filename,
        } => build_landscape(&source_file, &target_dir, &target_filename),
    }
}
