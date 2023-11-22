use std::{
    env,
    path::{self, PathBuf},
};

use alerty::Config;

fn main() {
    // check if there's a config file in the current path
    let config_file = path::Path::new("alerty.toml");
    let config_file = if config_file.exists() {
        config_file.to_owned()
    } else {
        // otherwise check $HOME/.config/alerty/config.toml
        let mut home_dir: PathBuf = env::var_os("HOME").map(PathBuf::from).unwrap();
        home_dir.push(".config");
        home_dir.push("alerty");
        home_dir.push("config.toml");
        home_dir
    };
    let config = Config::open(config_file).unwrap();
    alerty::run(&config).unwrap();
}
