# hello_cargo で学んだこと

## Cargo 周り
Cargo.toml に rand などの crate を書き込んで、 `cargo build`

- `cargo new` create a new rust project
- `cargo build` build the project
- `cargo run` build and run the project
- `cargo check` build check without generating binary
- `cargo build --release` build the project for release
- `cargo init` I used this by mistake :( This creates a new rust project in this directory

シャドーイングとmutが違うのが注意。シャドーイングはmutである必要はない。宣言後はまた変更不可になるからだ
上書き保存という言い方が近いかと。
むしろ、mutだと型は変えられない