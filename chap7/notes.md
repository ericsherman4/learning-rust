Module System
Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy of paths
Paths: A way of naming an item, such as a struct, function, or module

packages have .toml files, one or more crates. .toml describes the package
be a library crate (has a lib.rs). library crates dont produce an executable
crate root is main.rs for binary crate or lib.rs. You can also have additional binary crates by putting .rs files in src/bin 

this link is v good https:rust-book.cs.brown.edu/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet