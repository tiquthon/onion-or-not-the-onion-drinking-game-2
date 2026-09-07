use std::process::Command;

fn main() {
    println!(
        "cargo:rustc-env=BUILD_TIMESTAMP={}",
        chrono::Utc::now().to_rfc3339()
    );
    gather_git_information();
}

fn gather_git_information() {
    if cfg!(feature = "version_in_ui") {
        let repository =
            git2::Repository::open("..").expect("Should open the local git repository");

        let branch = repository
            .branches(None)
            .expect("Should get git branches")
            .flat_map(|r| r.ok())
            .find(|b| b.0.is_head())
            .expect("Should find the current git branch of HEAD")
            .0
            .name()
            .expect("Should get the name of the git branch")
            .map(ToOwned::to_owned)
            .expect("Should have a name of the git branch");

        let commit = repository
            .revparse_single(&branch)
            .expect("Should find head of branch")
            .peel_to_commit()
            .expect("Should find first commit");
        let id_long = commit.id().to_string();
        let id_short = id_long[0..7].to_string();
        let timestamp = commit.time().seconds();

        println!("cargo:rustc-env=BUILD_GIT_COMMIT_ID_SHORT={id_short}");
        println!("cargo:rustc-env=BUILD_GIT_COMMIT_ID_LONG={id_long}");
        println!("cargo:rustc-env=BUILD_GIT_COMMIT_TIMESTAMP={timestamp}");
        println!("cargo:rustc-env=BUILD_GIT_COMMIT_BRANCH={branch}");

        println!(
            "cargo:rustc-env=BUILD_GIT_DIRTY={}",
            if is_git_dirty() { "true" } else { "false" }
        );
    } else {
        println!(
            "cargo:warning=No git information will be included in this build. (Enable feature \"version_in_ui\" for this)"
        );
    }
}

fn is_git_dirty() -> bool {
    let staged_count_output = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--numstat")
        .output()
        .expect("Should run git diff --cached --numstat");
    assert!(staged_count_output.status.success());
    if !staged_count_output.stdout.is_empty() {
        return true;
    }

    let unstaged_count_output = Command::new("git")
        .arg("ls-files")
        .arg("--modified")
        .output()
        .expect("Should run git ls-files --modified");
    assert!(unstaged_count_output.status.success());
    if !unstaged_count_output.stdout.is_empty() {
        return true;
    }

    let untracked_count_output = Command::new("git")
        .arg("ls-files")
        .arg("--others")
        .arg("--exclude-standard")
        .output()
        .expect("Should run git ls-files --others --exclude-standard");
    assert!(untracked_count_output.status.success());
    if !untracked_count_output.stdout.is_empty() {
        return true;
    }

    false
}
