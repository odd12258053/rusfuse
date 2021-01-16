# rusfuse
Rust library for filesystems in userspace (FUSE ver3)

![Crates.io](https://img.shields.io/crates/l/rusfuse)
[![Crates.io](https://img.shields.io/crates/v/rusfuse.svg)](https://crates.io/crates/rusfuse)

# Dependencies
This `rusfuse` depend on [libfuse](https://github.com/libfuse/libfuse) with version 3.
To build `rusfuse` or any source that depend on it, `fuse` library needed.

## For Linux
### Install on ubuntu
```sh
$ apt install fuse3 libfuse3-dev
```

### Install on fedora
```sh
$ dnf install fuse3 fuse3-devel
```

# Usage
Write this in your `Cargo.toml`:

```toml
[dependencies]
rusfuse = "0.0.9"
```

Or, if you installed [cargo-edit](https://github.com/killercup/cargo-edit), you run this command:

```sh
$ cargo add rusfuse
```

To create a new filesystem, you implement the trait `rusfuse::FileSystem` for `struct` of your filesystem.
If you want more examples, you see a file in [examples](./examples). 
