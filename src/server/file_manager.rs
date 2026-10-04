use std::io;
use std::path::{Path, PathBuf};
use tokio::{
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
    sync::{mpsc, oneshot},
};

struct WriteChunk {
    pub data: Vec<u8>,
    pub file_name: PathBuf,
    pub offset: u64,
    pub reply: oneshot::Sender<Result<usize, io::Error>>,
}

struct ReadChunk {
    pub file_name: PathBuf,
    pub offset: u64,
    pub length: u64,
    pub reply: oneshot::Sender<Result<Vec<u8>, io::Error>>,
}

struct ReadHash {
    pub file_name: PathBuf,
    pub reply: oneshot::Sender<Result<String, io::Error>>,
}

#[derive(Clone)]
pub struct FileManager {
    ch: mpsc::Sender<FileAction>,
}

impl FileManager {
    pub async fn new<A: AsRef<Path>>(base_path: A) -> Self {
        let base_buf = PathBuf::from(base_path.as_ref());
        let (t_send, mut t_rec) = mpsc::channel(32);

        tokio::spawn(async move {
            while let Some(f_action) = t_rec.recv().await {
                match f_action {
                    FileAction::ReadChunk(rc) => {
                        let res = read_chunk(&rc, &base_buf).await;
                        _ = rc.reply.send(res);
                    }
                    FileAction::WriteChunk(wc) => {
                        let res = write_chunk(&wc, &base_buf).await;
                        _ = wc.reply.send(res);
                    }
                    FileAction::ReadHash(rh) => {
                        let full_path = base_buf.join(&rh.file_name);
                        let res = crate::hasher::hash_file_by_path(&full_path).await;
                        _ = rh.reply.send(res);
                    }
                }
            }
        });

        Self { ch: t_send }
    }

    pub async fn write_chunk(
        &self,
        data: Vec<u8>,
        file_name: PathBuf,
        offset: u64,
    ) -> Result<usize, io::Error> {
        let (in_send, in_recv) = oneshot::channel();
        self.ch
            .send(FileAction::WriteChunk(WriteChunk {
                data,
                file_name,
                offset,
                reply: in_send,
            }))
            .await
            .expect("FileManager dropped inner loop write_chunk");

        in_recv.await.expect("ONESHOT dropped inside WriteChunk")
    }

    pub async fn read_chunk(
        &self,
        file_name: PathBuf,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, io::Error> {
        let (in_send, in_recv) = oneshot::channel();
        self.ch
            .send(FileAction::ReadChunk(ReadChunk {
                file_name,
                offset,
                length,
                reply: in_send,
            }))
            .await
            .expect("File Manager Dropped Inner");

        in_recv.await.expect("ONESHOT dropped inside WriteChunk")
    }

    pub async fn read_hash(&self, file_name: PathBuf) -> Result<String, io::Error> {
        let (in_send, in_recv) = oneshot::channel();
        self.ch
            .send(FileAction::ReadHash(ReadHash {
                file_name,
                reply: in_send,
            }))
            .await
            .expect("File Manager Dropped Inner read_hash");

        in_recv.await.expect("ONESHOT dropped inside WriteChunk")
    }
}

pub enum FileAction {
    WriteChunk(WriteChunk),
    ReadChunk(ReadChunk),
    ReadHash(ReadHash),
}

async fn write_chunk(wc: &WriteChunk, base_path: &Path) -> Result<usize, io::Error> {
    let mut full_path = PathBuf::from(base_path);
    full_path.push(&wc.file_name);
    let mut file = tokio::fs::File::options()
        .write(true)
        .create(true)
        .open(&full_path)
        .await?;

    let _sk = file.seek(io::SeekFrom::Start(wc.offset)).await?;

    file.write(&wc.data).await
}

async fn read_chunk(rc: &ReadChunk, base_path: &Path) -> Result<Vec<u8>, io::Error> {
    let mut full_path = PathBuf::from(base_path);
    full_path.push(&rc.file_name);
    let mut file = tokio::fs::File::options()
        .read(true)
        .open(&full_path)
        .await?;

    let _sk = file.seek(io::SeekFrom::Start(rc.offset)).await?;

    let mut buf: Vec<u8> = Vec::with_capacity(rc.length as usize);
    let _n = file.take(rc.length).read_to_end(&mut buf).await?;

    Ok(buf)
}

#[cfg(test)]
pub mod test_file_manager {
    use super::*;
    use tokio;

    #[test]
    fn can_save_and_reload_the_same_data() {
        let rt = tokio::runtime::Runtime::new().expect("Could not get runtime for test");
        rt.block_on(async {
            let data_a = b"Hello all the people".to_vec();
            let a_len = data_a.len() as u64;
            let data_b = b" and everyone else".to_vec();
            let b_len = data_b.len() as u64;
            let filename = "test_can_save_server.txt";
            let f_man = FileManager::new(PathBuf::from("ungit/volatile")).await;

            let aw_a = f_man
                .write_chunk(data_a, PathBuf::from(filename), 0)
                .await
                .unwrap();
            let aw_b = f_man
                .write_chunk(data_b, PathBuf::from(filename), a_len)
                .await
                .unwrap();

            assert_eq!(aw_a + aw_b, (a_len + b_len) as usize);
        });
    }
}
