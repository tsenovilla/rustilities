[<img alt="CI Workflow" src="https://img.shields.io/github/actions/workflow/status/tsenovilla/rustilities/ci.yml?branch=main&style=for-the-badge&logo=github&label=CI" height="20">](https://github.com/tsenovilla/rustilities/actions/workflows/ci.yml)
[<img alt="Codecov" src="https://img.shields.io/codecov/c/github/tsenovilla/rustilities?style=for-the-badge&logo=codecov" height="20">](https://codecov.io/gh/tsenovilla/rustilities)
[<img alt="Crates.io" src="https://img.shields.io/crates/v/rustilities.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/rustilities)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rustilities-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/rustilities)

# Description 📖📚

This crate offers several functionalities that are not necessarily related to each other. 
Please refer to the specific documentation for each part of the crate to learn more about it.

[The crate docs](https://docs.rs/rustilities/latest/rustilities) should be considered the only source of truth for this crate usage.

### Features ⚙️

The crate splits its functionalities into several features, allowing the compilation of only the parts that are needed. 

### Highlight: universal test builder 🧪

Composable test harnesses with pay-per-use resources and guaranteed cleanup:

```rust
use rustilities::universal_test_builder::{
	builders::{RustupComponent, TempDir},
	universal_test_builder,
};

#[universal_test_builder({builder = TempDir}, {builder = RustupComponent})]
struct MyTestBuilder;

MyTestBuilder::default()
	.with_temp_dir(()) // request only what the test needs; the rest is never built
	.build()
	.execute(|context| {
		assert!(context.temp_dir().path().is_dir());
	});
// Context dropped: every built resource ran its cleanup.
```

Batteries included: temp dirs, throwaway Rust projects, rustup components, and
disposable Postgres containers (plain URL, diesel connections, applied migrations,
diesel-async pools) behind the `postgres`/`diesel`/`diesel-async` features.

Check Rust docs for further insights on the feature

# Contributing 🤝🚀

Any contribution is more than welcome! 🤝🦾 Just open a PR with your changes and it'll be considered 😸

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in the work by you shall be dual licensed as below, without any additional
terms or conditions.

# License 📄

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
