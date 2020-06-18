mod clone_repo;

pub use clone_repo::*;

// - RepoClone (`git clone`)
// - RepoInit (`git init`)
// - RepoBranch (`git branch|switch`)
// - RepoTag (`git tag|switch`)
// - RepoManager (`git add|mv|rm|restore|commit|rollback`)
// ```rs
// let clone = RepoClone::new();
// clone.set_hardlinks(hardlinks: CloneLocal);
// clone.set_bare(bare);
// clone.set_branch(branch);
// while let Some(e) = clone.clone(url, path).await { ... }
// ```
