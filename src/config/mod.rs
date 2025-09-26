use serde::Deserialize;
use std::{collections::HashMap, error::Error, path::Path};

#[derive(Deserialize, Debug)]
pub struct Config {
    games: HashMap<String, GameConfig>,
}

impl Config {
    pub fn list(&self) -> Vec<&str> {
        self.games.keys().map(|x| x.as_str()).collect()
    }

    pub fn track(&self) -> Result<(), Box<dyn Error>> {
        for i in self.games.values() {
            if i.is_active() {
                let src_path: &Path = Path::new(i.get_source());
                let dst_path: &Path = Path::new(i.get_destination());

                if !src_path.exists() {
                    #[cfg(unix)]
                    {
                        use std::fs;
                        use std::os::unix::fs as fs_ext;

                        if let Err(e) = fs::rename(dst_path, src_path) {
                            return Err(Box::new(e));
                        };

                        match fs_ext::symlink(src_path, dst_path) {
                            Ok(_) => println!("Succesfully tracked game saves of {}", i.get_name()),
                            Err(e) => println!("{}", e),
                        };
                    }
                } else {
                    println!("{} is already tracked", i.get_name());
                }
            }
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct GameConfig {
    name: String,
    source: String,
    destination: String,
    enabled: bool,
}

impl GameConfig {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    pub fn get_destination(&self) -> &str {
        &self.destination
    }

    pub fn is_active(&self) -> bool {
        self.enabled
    }
}
