use std::env;

// Check for other non-Rust files if they changed requiring a rebuild
fn main() -> Result<(), std::io::Error> {
    // We only care about kernel.ld for now. This env var used below is
    // directly exported from the Makefile.
    let env_var = "LAPWING_LD_SCRIPT_PATH";
    let ld_script_path: String = match env::var(env_var) {
        Ok(value) => value,
        Err(e) => panic!("{}: {}", env_var, e),
    };

    println!("cargo:rerun-if-changed={}", ld_script_path);
    Ok(())
}
