use async_std::task::spawn_blocking;
use git2::{Error, FetchOptions, RemoteCallbacks};
use git2::build::{RepoBuilder};
use std::path::PathBuf;

pub struct RepoClone {
    bare: bool,
    branch: String,
}

impl RepoClone {

    pub fn bare(&self) -> bool {
        self.bare
    }

    pub fn set_bare(&mut self, bare: bool) {
        self.bare = bare;
    }

    pub fn branch(&self) -> &str {
        &self.branch
    }

    pub fn set_branch<S>(&mut self, branch: S)
        where
        S: Into<String>,
    {
        self.branch = branch.into();
    }

    pub async fn clone<S, T>(&self, source: S, target: T) -> Result<(), Error>
        where
        S: Into<String>,
        T: Into<PathBuf>,
    {
        let source = source.into();
        let target = target.into();
        let bare = self.bare.clone();
        let branch = self.branch.clone();

        spawn_blocking(move || {

            let callbacks = RemoteCallbacks::new();
            // callbacks.transfer_progress(|progress| {
            //     println!("=> total_objects: {:?}", progress.total_objects()); // total
            //     println!("=> indexed_objects: {:?}", progress.indexed_objects()); // current
            //     println!("=> received_bytes: {:?}", progress.received_bytes());
            //     true
            // });
            // callbacks.credentials(|_url, username_from_url, _allowed_types| {
            //     Cred::ssh_key(
            //         username_from_url.unwrap(),
            //         None,
            //         std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            //         None,
            //     )
            // });
            // callbacks.certificate_check(|cert, data| {});

            let mut fo = FetchOptions::new();
            fo.remote_callbacks(callbacks);

            let mut builder = RepoBuilder::new();
            builder.bare(bare);
            builder.branch(&branch);
            builder.fetch_options(fo);
            builder.clone(&source, &target)?;

            Ok(())
        }).await
    }
}

impl Default for RepoClone {

    fn default() -> Self {
        Self {
            bare: false,
            branch: String::from("master"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[async_std::test]
    async fn clones_public() {
        let root = TempDir::new().unwrap().path().to_owned();
        let clone = RepoClone::default();
        clone.clone("https://github.com/xpepermint/async-gitlib", &root).await.unwrap();
        assert!(root.join("Cargo.toml").exists());
    }
}
