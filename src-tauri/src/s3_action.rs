use chrono::{DateTime, Utc};
use futures::stream::StreamExt;
use itertools::Itertools;
use s3::error::S3Error;
use s3::serde_types::Object;
use s3::Bucket;
use std::io;
use std::path::PathBuf;
use tauri::ipc::Channel;
use thiserror::Error;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct ObjectDetail {
    last_modified: DateTime<Utc>,
    pub(crate) key: String,
    size: u64,
    etag: String,
}

#[derive(Debug, Error)]
pub(crate) enum RuntimeError {
    #[error("The bucket is empty.")]
    EmptyBucket(),
    #[error("Failure in S3-related operation: {0}.")]
    S3(S3Error),
    #[error("{0}")]
    Io(io::Error),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct ChannelBytes {
    value: u64,
}

pub(crate) fn find_latest(objects: &Vec<ObjectDetail>) -> Result<Option<ObjectDetail>, S3Error> {
    if objects.is_empty() {
        return Ok(None);
    }
    let latest = objects
        .iter()
        .filter(|o| o.size > 0)
        .max_by_key(|o| o.last_modified.clone());
    Ok(latest.cloned())
}

fn get_keys(objects: &Vec<Object>) -> Vec<String> {
    objects.iter().map(|o| o.key.clone()).collect()
}

pub(crate) async fn pull_file(
    bucket: &Box<Bucket>,
    object: &ObjectDetail,
    file_path: &PathBuf,
    event: Channel<ChannelBytes>,
) -> Result<(), RuntimeError> {
    let mut stream = bucket
        .get_object_stream(object.key.clone())
        .await
        .map_err(|e| RuntimeError::S3(e))?;
    let mut file = tokio::fs::File::create(file_path)
        .await
        .map_err(|e| RuntimeError::Io(e))?;
    let mut accumulator: u64 = 0;
    while let Some(chunk) = stream.bytes().next().await {
        let chunk = chunk.map_err(|e| RuntimeError::S3(e))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| RuntimeError::Io(e))?;
        accumulator += chunk.len() as u64;
        event.send(ChannelBytes { value: accumulator }).unwrap(); // using channel
    }
    Ok(())
}

pub(crate) async fn push_file(
    bucket: &Box<Bucket>,
    file: &mut tokio::fs::File,
    target_key: &str,
) -> Result<(), RuntimeError> {
    let mut stream = bucket
        .put_object_stream(file, target_key)
        .await
        .map_err(|e| RuntimeError::S3(e))?;
    Ok(())
}

pub(crate) async fn list_files(
    delimiter: &Option<String>,
    bucket: &Box<Bucket>,
    prefix: &str,
    // preserved
) -> Result<Vec<ObjectDetail>, RuntimeError> {
    let result = bucket
        .list(prefix.to_string(), delimiter.clone())
        .await
        .map_err(|e| RuntimeError::S3(e))?;
    if result.is_empty() {
        return Err(RuntimeError::EmptyBucket());
    }
    let objects = result[0].contents.clone();
    let objects = objects
        .iter()
        .filter(|o| o.size > 0)
        .map(|o| ObjectDetail {
            last_modified: o.last_modified.parse().unwrap(),
            key: o.key.clone(),
            size: o.size,
            etag: o.e_tag.clone().unwrap(),
        })
        .sorted_by_key(|o| o.last_modified.clone())
        .collect::<Vec<_>>();
    Ok(objects)
}
