use async_std::task::spawn_blocking;
use git2::{Error, Repository, RepositoryInitOptions};
use std::path::PathBuf;

pub struct RepoInit {
    bare: bool,
    no_reinit: bool,
    no_dotgit_dir: bool,
    mkpath: bool,
    workdir_path: PathBuf,
    description: String,
    initial_head: String,
}

impl RepoInit {

    pub fn bare(&self) -> bool {
        self.bare
    }

    pub fn set_bare(&mut self, enable: bool) {
        self.bare = enable;
    }

    pub fn no_reinit(&self) -> bool {
        self.no_reinit
    }

    pub fn set_no_reinit(&mut self, enable: bool) {
        self.no_reinit = enable;
    }

    pub fn no_dotgit_dir(&self) -> bool {
        self.no_dotgit_dir
    }

    pub fn set_no_dotgit_dir(&mut self, enable: bool) {
        self.no_dotgit_dir = enable;
    }

    pub fn mkpath(&self) -> bool {
        self.mkpath
    }

    pub fn set_mkpath(&mut self, enable: bool) {
        self.mkpath = enable;
    }

    pub fn workdir_path(&self) -> &PathBuf {
        &self.workdir_path
    }

    pub fn set_workdir_path<P>(&mut self, path: P)
        where
        P: Into<PathBuf>,
    {
        self.workdir_path = path.into();
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description<D>(&mut self, desc: D)
        where
        D: Into<String>,
    {
        self.description = desc.into();
    }

    pub fn initial_head(&self) -> &str {
        &self.initial_head
    }

    pub fn set_initial_head<H>(&mut self, head: H)
        where
        H: Into<String>,
    {
        self.initial_head = head.into();
    }

    pub async fn init<T>(&self, target: T) -> Result<(), Error>
        where
        T: Into<PathBuf>,
    {
        let target = target.into();

        let mut opts = RepositoryInitOptions::new();
        opts.bare(self.bare);
        opts.no_reinit(self.no_reinit);
        opts.no_dotgit_dir(self.no_dotgit_dir);
        opts.mkpath(self.mkpath);
        opts.workdir_path(&self.workdir_path);
        opts.description(&self.description);
        opts.initial_head(&self.initial_head);

        spawn_blocking(move || {
            Repository::init_opts(target, &opts)?;
            Ok(())
        }).await
    }
}

impl Default for RepoInit {

    fn default() -> Self {
        Self {
            bare: false,
            no_reinit: false,
            no_dotgit_dir: false,
            mkpath: true,
            workdir_path: PathBuf::from("."),
            description: String::from("Unnamed repository"),
            initial_head: String::from("master"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[async_std::test]
    async fn initializes() {
        let target = TempDir::new().unwrap().path().to_owned();
        let init = RepoInit::default();
        init.init(&target).await.unwrap();
        assert!(target.join(".git").exists());
    }
}
