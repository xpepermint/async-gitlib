> Asynchronous library with libgit2 artifacts.

This package provides asynchronous structures for working with [libgit2](https://libgit2.org) and uses [async-std](https://github.com/async-rs/async-std) and [git2](https://docs.rs/git2/) under the hood.

**Example**

```rs
let root = PathBuf::from("./target");
let clone = RepoClone::default();
clone.set_bare(false);
clone.set_branch("master");
clone.clone("https://github.com/xpepermint/async-gitlib", &root).await?;
```
