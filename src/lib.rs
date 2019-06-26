pub mod helpers {
    use crypto::{digest::Digest, sha1::Sha1};
    use std::process::Command;

    pub fn latest_commit_hash(path: &str) -> String {
        String::from_utf8(
            Command::new("git")
                .args(&["-C", path, "log", "-1", "--format=%H"])
                .output()
                .unwrap_or_else(|_| panic!("no git"))
                .stdout,
        )
        .unwrap_or_else(|_| panic!("error in getting output"))
        .trim_end()
        .to_owned()
    }

    //fixme: confirm not added any changes
    pub fn empty_commit(path: &str, prefix: &str, name: &str) {
        Command::new("git")
            .env("GIT_COMMITTER_NAME", name)
            .env("GIT_COMMITTER_DATE", "Sun Jun 23 00:00:00 2018 +0900")
            .env("GIT_AUTHOR_DATE", "Sun Jun 23 00:00:00 2018 +0900")
            .args(&["-C", path, "commit", "-m", prefix, "--allow-empty"])
            .output()
            .unwrap_or_else(|_| panic!("no git"));
    }

    pub fn cat_file(path: &str, hash: &str) -> String {
        String::from_utf8(
            Command::new("git")
                .args(&["-C", path, "cat-file", "-p", hash])
                .output()
                .unwrap_or_else(|_| panic!("no git"))
                .stdout,
        )
        .unwrap_or_else(|_| panic!("error in getting output"))
        .to_owned()
    }

    // former & latter fixme: use regex
    pub fn split_base(cat_file: &str, committer_name: &str) -> (String, String) {
        let mut splitted = cat_file.split(committer_name);
        (
            splitted.next().unwrap().to_owned(),
            splitted.next().unwrap().to_owned(),
        )
    }

    pub fn hashize(seed: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.input_str(&format!(
            "commit {}\0{}",
            std::mem::size_of_val(seed.as_bytes()),
            seed
        ));
        hasher.result_str()
    }

    // ammendでもできるかも --committer-date-is-author-date
    pub fn filter_branch(path: &str, commit_hash: &str, committer_name: &str) {
        Command::new("git")
            .args(&[
                "-C",
                path,
                "filter-branch",
                "-f",
                "--env-filter",
                &format!(
                    r#"if [ $GIT_COMMIT = '{}' ]; then export GIT_COMMITTER_NAME='{}'; export GIT_COMMITTER_DATE='Sun Jun 23 00:00:00 2018 +0900'; export GIT_AUTHOR_DATE='Sun Jun 23 00:00:00 2018 +0900'; fi"#,
                    commit_hash, committer_name
                ),
                "HEAD^..HEAD"
            ])
            .output()
            .unwrap_or_else(|_| panic!("no git"));
    }
}
