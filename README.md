# web-image

A Rust library to facilitate easy interop between the Web Image APIs and `image-rs` crate via web assembly.

# Motivation

I wrote this crate for a use case where I needed to read and process images uploaded by the user but do so locally in
the browser. The crate facilitates converting web images to Rust's `image-rs` crate.