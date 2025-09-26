use git2::{AutotagOption, Cred, FetchOptions, RebaseOptions, RemoteCallbacks, Repository};
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

#[derive(Deserialize, Debug)]
pub struct Config {
    general: General,
    games: HashMap<String, GameConfig>,
}

impl Config {
    pub fn push(&self) {}

    pub fn pull(&self) {
        let repo = match Repository::open(Path::new(&self.general.repo)) {
            Ok(r) => r,
            Err(e) => panic!("Failed to open repo {}: {}", self.general.repo, e),
        };

        match Self::pull_rebase(&repo, "origin", "main") {
            Ok(_) => println!("Rebased successfully"),
            Err(e) => println!("{}", e),
        }
    }

    pub fn list(&self) -> Vec<&str> {
        self.games.keys().map(|x| x.as_str()).collect()
    }

    pub fn track(&self) {
        for i in self.games.values() {
            if i.enabled {
                let src_path: &Path = Path::new(&i.source); // file to be synced
                let dst_path: &Path = Path::new(&i.destination); // game file

                if !src_path.exists() {
                    #[cfg(unix)]
                    {
                        use std::fs;
                        use std::os::unix::fs as fs_ext;

                        match Self::get_metadata(dst_path, |x| x.is_symlink()) {
                            Ok(r) => {
                                if r {
                                    println!(
                                        "{}: Game file is a symlink, will not backup symlink",
                                        i.name
                                    );
                                    continue;
                                }
                            }
                            Err(e) => {
                                println!("{}: {} -> {}", i.name, dst_path.display(), e);
                                continue;
                            }
                        };

                        if let Err(_) = fs::rename(dst_path, src_path) {
                            println!(
                                "Error moving file from {} to {}",
                                dst_path.display(),
                                src_path.display()
                            );
                            println!("Skipping {}", i.name);

                            continue;
                        };

                        match fs_ext::symlink(src_path, dst_path) {
                            Ok(_) => println!("Succesfully tracked game saves of {}", i.name),
                            Err(e) => println!("{}", e),
                        };
                    }
                } else {
                    println!("{}: Game file is already tracked", i.name);
                }
            }
        }
    }

    fn get_metadata(
        path: &Path,
        predicate: fn(std::fs::FileType) -> bool,
    ) -> std::io::Result<bool> {
        use std::fs;
        let metadata = fs::symlink_metadata(path)?;
        let file_type = metadata.file_type();

        Ok(predicate(file_type))
    }

    fn pull_rebase(repo: &Repository, remote_name: &str, branch: &str) -> Result<(), git2::Error> {
        let mut remote = repo.find_remote(remote_name)?;

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username, _allowed_types| {
            // Cred::ssh_key(username.unwrap_or("git"), None, key, None)
            Cred::ssh_key_from_agent(username.unwrap_or("git"))
        });

        let mut fo = FetchOptions::new();
        fo.download_tags(AutotagOption::All);
        fo.remote_callbacks(callbacks);
        remote.fetch(&[branch], Some(&mut fo), None)?;

        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let annotated = repo.reference_to_annotated_commit(&fetch_head)?;

        let branch_annotated = repo.reference_to_annotated_commit(&repo.head()?)?;

        let mut ro = RebaseOptions::new();
        let mut rebase = repo.rebase(
            Some(&branch_annotated),
            None,
            Some(&annotated),
            Some(&mut ro),
        )?;

        while let Some(_op) = rebase.next() {
            let sig = repo.signature()?;
            rebase.commit(None, &sig, None)?;
        }

        rebase.finish(None)?;

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

#[derive(Deserialize, Debug)]
struct General {
    repo: String,
}
