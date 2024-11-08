use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use s3::{Bucket, Region};
use s3::error::S3Error;
use tauri_plugin_store::StoreExt;

#[derive(serde::Serialize)]
struct FileDetail {
    path: String,
    name: String,
    size: u64,
}

async fn init_bucket(app: tauri::AppHandle) -> Result<Box<Bucket>, S3Error> {
    let store = app.store("store.json").unwrap();
    let region = store.get("region").expect("Region not found").to_string();
    let access_key = store.get("access_key").expect("Access key not found").to_string();
    let secret_key = store.get("secret_key").expect("Secret key not found").to_string();
    let bucket_name = store.get("bucket_name").expect("Bucket name not found").to_string();
    let endpoint = store.get("endpoint").expect("Endpoint not found").to_string();
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
async fn upload_file(app: tauri::AppHandle, file: FileDetail) {
    if let Ok(bucket) = init_bucket(app).await {

    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, pick_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
