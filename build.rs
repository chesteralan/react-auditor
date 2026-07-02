fn main() {
    // Expose the version so tests can verify --version output
    println!("cargo::rustc-env=REACT_AUDITOR_VERSION={}", env!("CARGO_PKG_VERSION"));
}
