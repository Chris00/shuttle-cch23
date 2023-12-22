use std::{
    cmp::Reverse,
    fmt::Debug,
    path::{Path, PathBuf},
    fs::File,
    io::{BufRead, BufReader}
};
use axum::{
    body::Bytes, http::StatusCode,
};
use tar::Archive;
use bytes::Buf;
use git2::{Repository, build::CheckoutBuilder, TreeWalkResult};

pub async fn cookie(body: Bytes) -> Result<String, (StatusCode, String)> {
    let mut tar = Archive::new(body.reader());
    fn err(e: impl Debug) -> (StatusCode, String) {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))
    }
    let dst = tempfile::tempdir().map_err(err)?;
    tar.unpack(&dst).map_err(err)?;
    get_cookie(&dst).await.map_err(err)
}

// https://libgit2.org/docs/guides/101-samples/
async fn get_cookie(dst: impl AsRef<Path>) -> eyre::Result<String> {
    let git = Repository::open(&dst)?;
    git.set_head("refs/heads/christmas")?;
    let mut rw = git.revwalk()?;
    rw.push_head()?;
    let mut commits = vec![];
    for oid in rw {
        let oid = oid?;
        let commit = git.find_commit(oid)?;
        commit.tree()?.walk(git2::TreeWalkMode::PreOrder,
            |dir, entry| {
                let name = entry.name().unwrap();
                if name == "santa.txt" {
                    git.checkout_tree(commit.as_object(), Some(
                        CheckoutBuilder::new().force())).unwrap();
                    let mut fname = PathBuf::new();
                    fname.push(&dst);
                    fname.push(dir);
                    fname.push(name);
                    let fh = File::open(fname).unwrap();
                    let fh = BufReader::new(fh);
                    for l in fh.lines() {
                        let l = l.unwrap();
                        if l.contains("COOKIE") {
                            commits.push(commit.clone());
                            return TreeWalkResult::Abort;
                        }
                    }
                }
                TreeWalkResult::Ok})?;
    }
    commits.sort_by_key(|c| Reverse(c.time()));
    if let Some(commit) = commits.first() {
        let author = commit.author();
        let author = author.name().unwrap_or("");
        Ok(format!("{} {}", author, commit.id()))
    } else {
        Ok("No such commit".to_string())
    }
}

/// Returns the number of files inside the tar file.
pub async fn archive_files(body: Bytes) -> String {
    let mut tar = Archive::new(body.reader());
    let c = tar.entries().unwrap()
        .filter(|f| f.is_ok())
        .count();
    format!("{}", c)
}

/// Returns the sum of all file sizes inside the tar file.
pub async fn archive_files_size(body: Bytes) -> String {
    let mut tar = Archive::new(body.reader());
    let c: u64 = tar.entries().unwrap()
        .filter_map(|f| {
            if let Ok(f) = f {
                Some(f.size())
            } else {
                None
            }
        })
        .sum();
    format!("{}", c)
}
