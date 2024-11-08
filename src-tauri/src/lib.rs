mod s3_action;

use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use s3::{Bucket, Region};
use s3::error::S3Error;
use s3_action::push_file;
use tauri_plugin_store::StoreExt;

use tauri::{AppHandle, Emitter};
use crate::s3_action::{find_latest, list_files, ObjectDetail, RuntimeError};

#[derive(serde::Serialize, serde::Deserialize)]
struct FileDetail {
    path: String,
    name: String,
    size: u64,
}

async fn init_bucket(app: &AppHandle) -> Result<Box<Bucket>, S3Error> {
    let store = app.store("store.json").unwrap();
    // need to trim quotes as store won't deserialize for JSON
    let region = store.get("region").expect("Region not found").to_string().replace("\"", "");
    let access_key = store.get("access_key").expect("Access key not found").to_string().replace("\"", "");
    let secret_key = store.get("secret_key").expect("Secret key not found").to_string().replace("\"", "");
    let bucket_name = store.get("bucket_name").expect("Bucket name not found").to_string().replace("\"", "");
    let endpoint = store.get("endpoint").expect("Endpoint not found").to_string().replace("\"", "");
    let region = Region::Custom {
        region,
        endpoint,
    };
    let credentials = s3::creds::Credentials::new(
        Option::from(access_key.as_str()),
        Option::from(secret_key.as_str()),
        None, None, None)?;
    let s3 = Bucket::new(&bucket_name, region, credentials);
    s3
}

// #[cfg(not(target_os = "linux"))]
#[tauri::command]
async fn pick_file(app: tauri::AppHandle) -> Option<FileDetail> {
    let file_handle = rfd::AsyncFileDialog::new().pick_file().await?; // return None if user cancels
    let path = file_handle.path();
    Some(FileDetail {
        path: path.to_str().unwrap().to_string(),
        name: path.file_name().unwrap().to_str().unwrap().to_string(),
        size: fs::metadata(&path).unwrap().len(),
    })
}

#[tauri::command]
async fn peek_latest_file(app: tauri::AppHandle) -> Option<ObjectDetail> {
    if let Ok(bucket) = init_bucket(&app).await {
        let list_result;
        match list_files(&None, &bucket, "/").await {
            Ok(result) => { list_result = result; }
            Err(e) => {
                app.emit("glob_error", e.to_string());
                return None;
            }
        }
        // fetch the latest file
        match find_latest(&list_result) {
            Ok(Some(obj)) => {
                Some(obj)
            }
            Ok(None) => {
                app.emit("glob_error", "No file found currently. Maybe you need to `send` first?");
                None
            }
            Err(e) => {
                app.emit("glob_error", e.to_string());
                None
            }
        }
    } else {
        None
    }
}

#[tauri::command]
async fn upload_file(app: tauri::AppHandle, file: FileDetail) {
    if let Ok(bucket) = init_bucket(&app).await {
        let path = PathBuf::from(file.path);
        if !path.exists() {
            eprintln!("File does not exist: {:?}", &path);
            return;
        }
        let mut file = tokio::fs::File::open(&path).await
            .expect("Unable to open this file. Maybe you are occupying the file or have no permission to access it?");
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let mut key = file_name;
        app.emit("upload_started", 0).expect("Failed to emit event");
        // calculate time cost
        let start_time = std::time::Instant::now();
        match push_file(
            &bucket,
            &mut file,
            &key,
        ).await {
            Ok(_) => {
                let end_time = std::time::Instant::now();
                let time_cost = (end_time - start_time).as_secs_f32();
                app.emit("upload_success", time_cost).expect("Failed to emit event");
                return;
            }
            Err(e) => {
                app.emit("upload_failed", e.to_string()).expect("Failed to emit event");
                eprintln!("{e:?}");
                return;
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![pick_file, upload_file, peek_latest_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
