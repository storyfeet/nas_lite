use chrono::{NaiveDate, Utc};
use err_tools::traceable::{TraceError, TraceResult};
use tokio::io::Error as IOError;

type FolderResult = Result<FolderData, IOError>;

pub enum FileType {
    File,
    Folder,
    Link,
}

pub struct FolderData {
    pub f_type: FileType,
    pub hash: String,
    pub name: String,
    pub files: Vec<FolderData>,
    pub modified: NaiveDate,
    pub errors: Option<TraceError>,
}

pub async fn walk_folder(path: &str, root_name: String) -> FolderResult {
    let root_meta = tokio::fs::metadata(path).await?;
    if root_meta.is_file() {}
    Ok(FolderData {
        f_type: FileType::File,
        hash: "HHH".to_string(),
        name: "name".to_string(),
        files: vec![],
        modified: chrono::Utc::now().date_naive(),
        errors: None,
    })
}

pub async fn hash_file(path: &str) {}
