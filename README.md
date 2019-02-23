# with-clone-rs
## Rust procedural macro to simplify cloning a variable for closures, blocks, expressions, etc.

Read `tests/main.rs` for usage examples. N.b. `CloneTracker` is just used to verify things have been cloned. There is no need to reimplement it in your own code.

Uses procedural macros so that the attribute syntax can be used.

Does not correctly parse the attribute TokenStream, just does the bare minimum to make this work. May produce starnge output in some edge case I've not considered.

MIT License, see `LICENSE`
