[![github]](https://github.com/usagi/build-pretty)&ensp;[![crates-io]](https://crates.io/crates/build-pretty)&ensp;[![docs-rs]](https://docs.rs/build-pretty)<br>

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

<img src="https://imgur.com/yZIc3WP.png" alt="image1">

# build-pretty

This lib crate is one of workarround solution for log displaying of [Build Scripts]() issue such as [rust-lang/cargo#985]().

[Build Scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[rust-lang/cargo#985]: https://github.com/rust-lang/cargo/issues/985

## Usage

1. Add `build-pretty` dependency in `[build-dependencies]` of your project's `Cargo.toml`

```toml
[package]
...
build = "build.rs"
...
[build-dependencies]
build-pretty = "*"
```

2. Write your `build.rs` with `build-pretty`:

```rust
use build_pretty::{
 build_pretty,
 CommandBuilder
};

fn main()
{
 build_pretty!()
  .enque_command("Drink a cup of tea", CommandBuilder::new_with_arg("echo", "🍵 Green!\n☕ Black!\n🧋 Bubbles!").into())
  .enque_command("Eat a hotdog", CommandBuilder::new_with_arg("echo", "🌭 Hotdog!\n♨️ Hot?\n🐕 Dog!\n🌶️ Hot?\n🐶 Dog?").into())
  .enque_command("ls -l -a", CommandBuilder::new_with_args("ls", &["-l", "-a"]).into())
  .enque_fn("Ofcourse Fn can be used", Box::new(|output|{ *output = "brabrabra\nmewmewmew\nnekonyankonyanko🐾".to_string(); Ok(()) }))
  .enque_command("Sleep", CommandBuilder::new_with_arg("echo", "😴 I'm sleee....\n💤...\n🛌....pyyyyy....").into());
}
```

and then:

<img src="https://imgur.com/yZIc3WP.png" alt="image1">

## Examples:

- [examples/might_be_complete/](examples/might_be_complete/)
- [examples/might_be_safe_panic/](examples/might_be_safe_panic/)

## Features:

- `std::process::Command` runner with STDOUT/STRERR log capturing!
- `Fn` runner with `String` output capturing!
- `cargo_warning_ln!` macro, it's also to Build Script version `println!`.
- runtime message customize feature. on/off, change message detail as your like with easy template keywords.
- i18n subsystem support. (using [rust-i18n](https://crates.io/crates/rust-i18n))
  - Currently supported: en ja
  - Welcome your additional language support PR!😆

## LICENSE

- [MIT](LICENSE.md)

## Author

- USAGI.NETWORK / Usagi Ito <https://github.com/usagi/>
