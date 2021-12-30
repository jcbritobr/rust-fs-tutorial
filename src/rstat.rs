use std::path::PathBuf;

use errors::StatsError;
use statistics::get_summary_src_stats;
use structopt::StructOpt;

mod errors;
mod statistics;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rstat",
    about = "A tool to generate rust code statistics" 
)]
struct Opt {
    #[structopt(name = "source directory", parse(from_os_str))]
    in_dir: PathBuf,
    #[structopt(name = "mode", short)]
    mode: String,
}

fn main() -> Result<(), StatsError> {
    let opt = Opt::from_args();
    let mode = &opt.mode[..];
    match mode {
        "src" => {
            let stats = get_summary_src_stats(&opt.in_dir)?;
            println!("Summary stats: {:?}", stats);
        }
        _ => println!("No statiscs"),
    }

    Ok(())
}