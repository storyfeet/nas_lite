use chrono::NaiveDateTime;
use std::path::PathBuf;
use tokio::io::Error as IOError;

type FolderResult = Result<FolderData, IOError>;

#[derive(Debug)]
pub enum FileType {
    File,
    Folder,
    Link,
}

#[derive(Debug)]
pub struct FolderData {
    pub f_type: FileType,
    pub hash: String,
    pub name: String,
    pub files: Vec<FolderData>,
    pub modified: NaiveDateTime,
    pub errors: Vec<tokio::io::Error>,
}

pub fn walk_folder(path: PathBuf, root_name: String) -> impl Future<Output = FolderResult> + Send {
    // Note the inner async block, is a workaround to handle the recursive call.
    // as otherwise the autotrait, Send is not assigned to the future
    // FROM : https://stackoverflow.com/questions/78990686/recursive-async-function-future-cannot-be-sent-between-threads-safely
    async {
        let pbuf = path;
        let root_meta = tokio::fs::metadata(&pbuf).await?;
        if root_meta.is_file() {
            let hash = crate::hasher::hash_file_by_path(&pbuf).await?;
            return Ok(FolderData {
                f_type: FileType::File,
                hash,
                name: root_name,
                files: Vec::new(),
                modified: root_meta
                    .modified()
                    .map(system_time_to_naive_date)
                    .expect("file has no modified date"),
                errors: Vec::new(),
            });
        }

        if root_meta.is_dir() {
            let mut dir = tokio::fs::read_dir(&pbuf).await?;
            let mut handles = Vec::new();
            while let Some(entry) = dir.next_entry().await? {
                let inner_path = pbuf.join(entry.file_name());
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
            let hashes: Vec<&str> = children.iter().map(|c| c.hash.as_str()).collect();

            let folder_hash = crate::hasher::hash_hashes(&hashes);

            return Ok(FolderData {
                f_type: FileType::File,
                hash: folder_hash,
                name: root_name,
                files: children,
                modified: root_meta
                    .modified()
                    .map(system_time_to_naive_date)
                    .expect("file has no modified date"),
                errors: errs,
            });
        }

        Err(tokio::io::Error::new(
            tokio::io::ErrorKind::Unsupported,
            err_tools::SError("Not a file or folder"),
        ))
    }
}

fn system_time_to_naive_date(sys_time: std::time::SystemTime) -> chrono::NaiveDateTime {
    let nanos = match sys_time.duration_since(std::time::UNIX_EPOCH) {
        Ok(dur) => dur.as_nanos(),
        Err(e) => e.duration().as_nanos(),
    };
    chrono::DateTime::from_timestamp_nanos(nanos as i64).naive_utc()
}
