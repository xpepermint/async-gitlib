> Asynchronous library with libgit2 artifacts.

This package provides asynchronous structures for working with [libgit2](https://libgit2.org) features and uses [async-std](https://github.com/async-rs/async-std) and [git2](https://docs.rs/git2/) under the hood.

**Clone**

```rs
let source = "https://github.com/xpepermint/async-gitlib";
let target = "./target";
let task = RepoClone::default();
task.set_bare(false);
task.set_branch("master");
task.clone(source, target).await?;
```

**Init**

```rs
let target = "./target";
let task = RepoInit::default();
task.set_bare(false);
task.init(target).await?;
```
