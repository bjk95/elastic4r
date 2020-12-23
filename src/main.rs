use structopt::StructOpt;
mod config;

fn main() {
    let args = Cli::from_args();

    let elastic_config = config::read_config(&args.config_path);

    println!("{:?}", args);
    println!("{:?}", elastic_config)
}

#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    query: String,

    #[structopt(parse(from_os_str))]
    #[structopt(short = "c", long = "config", default_value = "config/example-conf-brad.yaml")]
    config_path: std::path::PathBuf
}
