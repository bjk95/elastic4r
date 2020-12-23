use structopt::StructOpt;
mod config;

fn main() {
    let args = Cli::from_args();

    config::read_config(&args.config_path);

    println!("{:?}", args);
    // println!
}

#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    query: String,

    #[structopt(parse(from_os_str))]
    #[structopt(short = "c", long = "config")]
    config_path: std::path::PathBuf
}
