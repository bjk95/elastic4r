use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    elastic_url: String,
    username: Option<String>,
    password: Option<String>
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self { Self { 
        elastic_url: "empty".into(),
        username: None,
        password: None 
    } }
}

pub fn read_config(path: &std::path::PathBuf) -> Config {
    let config_path = path.to_path_buf().into_os_string().into_string().unwrap();
    println!("{}", config_path);

    let cfg: Config = confy::load_path(&config_path).expect("No config found");

    cfg
}

