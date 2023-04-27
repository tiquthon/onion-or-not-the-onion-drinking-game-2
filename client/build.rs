use last_git_commit::LastGitCommit;

fn main() {
    println!("cargo:rerun-if-changed=../.git");
    gather_git_information();
}

fn gather_git_information() {
    if cfg!(feature = "version_in_ui") {
        let lgc = LastGitCommit::new()
            .set_path("..")
            .build()
            .expect("Failed getting last git commit.");
        println!(
            "cargo:rustc-env=BUILD_GIT_COMMIT_ID_SHORT={}",
            lgc.id().short()
        );
        println!(
            "cargo:rustc-env=BUILD_GIT_COMMIT_ID_LONG={}",
            lgc.id().long()
        );
        println!(
            "cargo:rustc-env=BUILD_GIT_COMMIT_TIMESTAMP={}",
            lgc.timestamp()
        );
        println!("cargo:rustc-env=BUILD_GIT_COMMIT_BRANCH={}", lgc.branch());

        println!(
            "cargo:rustc-env=BUILD_TIMESTAMP={}",
            chrono::Utc::now().to_rfc3339()
        );
    } else {
        println!("cargo:warning=No git information will be included in this build. (Enable feature \"version_in_ui\" for this)");
    }
}
