use std::path::Path;

pub fn keccak_path() -> &'static Path {
    Path::new(concat!(
        env!("OUT_DIR"),
        "/wasm32-unknown-unknown/release/",
        "keccak.wasm"
    ))
}
