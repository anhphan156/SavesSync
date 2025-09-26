use crate::constants::*;
use directories::ProjectDirs;
use std::fs::File;

pub fn get_config() -> std::io::Result<File> {
    if let Some(proj_dirs) = ProjectDirs::from("com", DEVELOPER, PROJECT) {
        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join(SETTINGS_FILE);

        if config_file.exists() {
            return File::open(config_file);
        } else {
            if !config_dir.exists() {
                std::fs::create_dir_all(config_dir)?
            }

            return std::fs::File::create(config_file);
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Config directory not found",
    ))
}
