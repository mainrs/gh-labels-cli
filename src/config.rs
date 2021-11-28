use directories_next::ProjectDirs;
use std::path::PathBuf;

const CONFIG_FILENAME: &str = "labels.json";

fn config_dir() -> Option<PathBuf> {
    ProjectDirs::from("net.zerotask", "Zerotask", "gh-labels-cli")
        .map(|v| v.config_dir().to_path_buf())
}

pub fn config_file() -> Option<PathBuf> {
    config_dir().map(|p| p.join(CONFIG_FILENAME))
}
