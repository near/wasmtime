use std::process::Command;

type Error = Box<dyn std::error::Error>;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), Error> {
    build_contract("keccak", &[])?;
    Ok(())
}

fn build_contract(dir: &str, args: &[&str]) -> Result<(), Error> {
    let target_dir = out_dir();

    let mut cmd = cargo_build_cmd(&target_dir);
    cmd.args(args);
    cmd.current_dir(dir);
    check_status(cmd)?;

    println!("cargo:rerun-if-changed=./{}/src/lib.rs", dir);
    println!("cargo:rerun-if-changed=./{}/Cargo.toml", dir);
    Ok(())
}

fn cargo_build_cmd(target_dir: &std::path::Path) -> Command {
    let mut res = Command::new("cargo");

    res.env_remove("CARGO_BUILD_RUSTFLAGS");
    res.env_remove("CARGO_ENCODED_RUSTFLAGS");
    res.env_remove("RUSTC_WORKSPACE_WRAPPER");

    res.env("RUSTFLAGS", "-Clink-arg=-zstack-size=2048");
    res.env("CARGO_TARGET_DIR", target_dir);

    res.args(["build", "--target=wasm32-unknown-unknown", "--release"]);

    res
}

fn check_status(mut cmd: Command) -> Result<(), Error> {
    cmd.status()
        .map_err(|err| format!("command `{cmd:?}` failed to run: {err}"))
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(format!(
                    "command `{cmd:?}` exited with non-zero status: {status:?}"
                ))
            }
        })
        .map_err(Error::from)
}

fn out_dir() -> std::path::PathBuf {
    std::env::var("OUT_DIR").unwrap().into()
}
