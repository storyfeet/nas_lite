use chrono::NaiveDateTime;
use std::borrow::Cow;
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use tokio::io::Error as IOError;

use iter_tools::Itertools;

type FolderResult = Result<FolderData, IOError>;

#[derive(Debug)]
pub enum Content {
    File(PathBuf),
    Folder(FolderContent),
    Link(PathBuf),
}

/**
 * Only what is in the folder, and is relevant for the hash
 */
#[derive(Debug)]
pub struct FolderContent {
    oks: Vec<FolderData>,
    errs: Vec<tokio::io::Error>,
}

#[derive(Debug)]
pub struct FolderData {
    pub hash: String,
    pub name: String,
    pub content: Content,
    pub modified: NaiveDateTime,
}

pub fn content(
    pathbuf: PathBuf,
    meta: &Metadata,
) -> impl Future<Output = Result<Content, IOError>> + Send {
    async {
        let path = pathbuf.as_path();
        if meta.is_file() {
            return Ok(Content::File(pathbuf));
        }

        if meta.is_dir() {
            let mut dir = tokio::fs::read_dir(path).await?;
            let mut handles = Vec::new();
            while let Some(entry) = dir.next_entry().await? {
                let inner_path = path.join(entry.file_name());
                let inner_name = entry.file_name().to_string_lossy().to_string();
                handles.push(tokio::spawn(walk_folder(inner_path, inner_name)));
            }

            let mut children: Vec<FolderData> = Vec::new();
            let mut errs = Vec::new();
            for handle in handles {
                match handle.await? {
                    Ok(dat) => children.push(dat),
                    Err(e) => errs.push(e),
                }
            }

            return Ok(Content::Folder(FolderContent {
                oks: children,
                errs: errs,
            }));
        }

        Err(tokio::io::Error::new(
            tokio::io::ErrorKind::Unsupported,
            err_tools::SError("Not a file or folder"),
        ))
    }
}

pub fn walk_folder(path: PathBuf, root_name: String) -> impl Future<Output = FolderResult> + Send {
    // Note the inner async block, is a workaround to handle the recursive call.
    // as otherwise the autotrait, Send is not assigned to the future
    // FROM : https://stackoverflow.com/questions/78990686/recursive-async-function-future-cannot-be-sent-between-threads-safely
    async {
        let root_meta = tokio::fs::metadata(&path).await?;
        let f_content = content(path, &root_meta).await?;
        let hash = f_content.to_hash().await; // This may need a try later on path

        Ok(FolderData {
            content: f_content,
            hash: hash,
            name: root_name,
            modified: root_meta
                .modified()
                .map(system_time_to_naive_date)
                .expect("file has no modified date"),
        })
    }
}

fn system_time_to_naive_date(sys_time: std::time::SystemTime) -> chrono::NaiveDateTime {
    let nanos = match sys_time.duration_since(std::time::UNIX_EPOCH) {
        Ok(dur) => dur.as_nanos(),
        Err(e) => e.duration().as_nanos(),
    };
    chrono::DateTime::from_timestamp_nanos(nanos as i64).naive_utc()
}

pub fn represent_children(children: &[FolderData]) -> String {
    let mut files: Vec<&FolderData> = children.iter().collect();

    files.sort_by(|a, b| a.name.cmp(&b.name));

    files
        .iter()
        .map(|fd| format!("{},{}", fd.hash, fd.name))
        .join("\n")
}

impl Content {
    pub async fn to_hash(&self) -> String {
        match self {
            Content::File(pbuf) => crate::hasher::hash_file_by_path(pbuf)
                .await
                .expect("Hashing file doesn;t exist"),
            Content::Folder(c) => crate::hasher::hash_bytes(c.to_rep_string().as_bytes()),
            Content::Link(_) => unimplemented!("Sim links will be a challenge"),
        }
    }

    pub async fn to_rep_bytes(&self) -> Vec<u8> {
        match self {
            Content::File(pbuf) => tokio::fs::read(pbuf).await.unwrap_or(b"no_bytes".to_vec()),
            Content::Folder(c) => c.to_rep_string().as_bytes().to_vec(),
            Content::Link(_) => unimplemented!("Sim links will be a challenge"),
        }
    }
}

impl FolderContent {
    pub fn to_rep_string(&self) -> String {
        let mut files: Vec<&FolderData> = self.oks.iter().collect();

        files.sort_by(|a, b| a.name.cmp(&b.name));

        files
            .iter()
            .map(|fd| format!("{},{}", fd.hash, fd.name))
            .join("\n")
    }
}
