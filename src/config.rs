use serde::{Serialize, Deserialize};
use std::io;
use std::fs;


#[derive(Debug, Serialize, Deserialize)]
struct Config {
    elastic_url: String
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self { Self { elastic_url: "empty".into() } }
}

pub fn read_config(path: &std::path::PathBuf) -> Result<(), io::Error>  {
    let config_path = path.to_path_buf().into_os_string().into_string().unwrap();
    println!("{}", config_path);

    let cfg: Config = confy::load(&config_path).unwrap() ;



    println!("{:?}", cfg);
    Ok(())
}

