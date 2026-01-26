//! Build script to install git hooks via rhusky
fn main() {
    rhusky::Rhusky::new()
        .hooks_dir(".githooks")
        .skip_in_env("GITHUB_ACTIONS")
        .install()
        .ok();
}
