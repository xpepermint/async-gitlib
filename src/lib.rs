mod repo_clone;
mod repo_init;

pub use git2::{Error, ErrorClass, ErrorCode};
pub use repo_clone::*;
pub use repo_init::*;

// - RepoClone (`git clone`)
// - RepoInit (`git init`)
// - RepoBranch (`git branch|switch`)
// - RepoTag (`git tag|switch`)
// - RepoStage (`git add|mv|rm|restore|commit|rollback`)
// ```rs
// let clone = RepoClone::new();
// clone.set_hardlinks(hardlinks: CloneLocal);
// clone.set_bare(bare);
// clone.set_branch(branch);
// while let Some(e) = clone.clone(url, path).await { ... }
// ```
