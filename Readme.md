# system requirements
1) rustc -V
rustc 1.81.0 (eeb90cda1 2024-09-04)

2) rustup -V
rustup 1.27.1 (54dd3d00f 2024-04-24)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.81.0 (eeb90cda1 2024-09-04)`

3) cargo -V
cargo 1.81.0 (2dbb1af80 2024-08-20)


# download

cargo clean
cargo build
cargo test

# run Note
cargo run --bin json_parser_app -- [strings | colors] [path_str]

Note: path_str example => C:/Users OR C:\\Users


# run Json Strings
cargo run --bin json_parser_app strings C:/Users/....../json_parser/resources/strings.json

# run Json Colors
cargo run --bin json_parser_app colors C:/Users/....../json_parser/resources/colors.json
