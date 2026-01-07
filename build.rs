//! Build script to install git hooks via sloughi
use sloughi::Sloughi;

fn main() {
    let _ = Sloughi::new()
        .custom_path(".githooks")
        .ignore_env("CI")
        .ignore_env("GITHUB_ACTIONS")
        .install();
}
