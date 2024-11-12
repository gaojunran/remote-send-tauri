mod s3_action;

use s3::error::S3Error;
use s3::{Bucket, Region};
use s3_action::push_file;
use std::{fs, io};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use rfd::FileHandle;
use tauri_plugin_store::StoreExt;

use crate::s3_action::{find_latest, list_files, pull_file, ChannelBytes, ObjectDetail};
use tauri::ipc::Channel;
use tauri::{AppHandle, Emitter};
use tauri_plugin_autostart::MacosLauncher;
use zip::write::{SimpleFileOptions};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FileDetail {
    path: String,
    name: String,
    size: u64,
}

impl FileDetail {
    fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let path_buf = path.as_ref();
        Self {
            path: path_buf.to_str().unwrap().to_string(),
            name: path_buf.file_name().unwrap().to_str().unwrap().to_string(),
            size: fs::metadata(&path_buf).unwrap().len(),
        }
    }

    fn default() -> Self {
        Self {
            path: "".to_string(),
            name: "".to_string(),
            size: 0,
        }
    }
}
async fn init_bucket(app: &AppHandle) -> Result<Box<Bucket>, S3Error> {
    let store = app.store("store.json").unwrap();
    // need to trim quotes as store won't deserialize for JSON
    let region = store
        .get("region")
        .expect("Region not found")
        .to_string()
        .replace("\"", "");
    let access_key = store
        .get("access_key")
        .expect("Access key not found")
        .to_string()
        .replace("\"", "");
    let secret_key = store
        .get("secret_key")
        .expect("Secret key not found")
        .to_string()
        .replace("\"", "");
    let bucket_name = store
        .get("bucket_name")
        .expect("Bucket name not found")
        .to_string()
        .replace("\"", "");
    let endpoint = store
        .get("endpoint")
        .expect("Endpoint not found")
        .to_string()
        .replace("\"", "");
    let region = Region::Custom { region, endpoint };
    let credentials = s3::creds::Credentials::new(
        Option::from(access_key.as_str()),
        Option::from(secret_key.as_str()),
        None,
        None,
        None,
    )?;
    let s3 = Bucket::new(&bucket_name, region, credentials);
    s3
}

#[cfg(all(not(target_os = "android"), not(target_os = "ios")))]  // pc only
#[tauri::command]
async fn pick_files(_app: AppHandle) -> Vec<FileDetail> {
    let file_handle = rfd::AsyncFileDialog::new().pick_files().await;
    let file_handle: Vec<FileHandle> = file_handle.unwrap_or_else(|| Vec::new());
    file_handle.iter().map(|file| {
        FileDetail::from_path(file.path())
    }).collect()
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[tauri::command]
async fn pick_files(app: tauri::AppHandle) -> Vec<FileDetail> {
    let files = app.dialog().file().blocking_pick_files().unwrap_or_default();
    files.iter().map(|&file| {
        let path = file.into_path();
        FileDetail {
            path: path.clone().unwrap().to_str().unwrap().to_string(),
            name: path.clone().unwrap().file_name().unwrap().to_str().unwrap().to_string(),
            size: fs::metadata(path.clone().unwrap()).unwrap().len(),
        }
    }).collect()
}

#[tauri::command]
async fn peek_latest_file(app: tauri::AppHandle) -> Option<ObjectDetail> {
    if let Ok(bucket) = init_bucket(&app).await {
        let list_result = match list_files(&None, &bucket, "").await {
            Ok(result) => result,
            Err(e) => {
                app.emit("glob_error", e.to_string()).unwrap();
                return None;
            }
        };
        // fetch the latest file
        match find_latest(&list_result) {
            Ok(Some(obj)) => Some(obj),
            Ok(None) => {
                app.emit(
                    "glob_error",
                    "No file found currently. Maybe you need to `send` first?",
                ).unwrap();
                None
            }
            Err(e) => {
                app.emit("glob_error", e.to_string()).unwrap();
                None
            }
        }
    } else {
        None
    }
}

#[tauri::command]
async fn upload_file(app: AppHandle, file: FileDetail) {
    if let Ok(bucket) = init_bucket(&app).await {
        let path = PathBuf::from(file.path);
        if !path.exists() {
            app.emit("upload_failed", "File does not exist!".to_string()).unwrap();
            return;
        }
        let mut file = match tokio::fs::File::open(&path).await {
            Ok(file) => { file }
            Err(_) => {
                app.emit("upload_failed",
                         "Unable to open this file. Maybe you are occupying the file or have no permission to access it?".to_string()).unwrap();
                return;
            }
        };
        let key = path.file_name().unwrap().to_str().unwrap();
        app.emit("upload_started", 0).unwrap();
        // calculate time cost
        let start_time = std::time::Instant::now();
        match push_file(&bucket, &mut file, &key).await {
            Ok(_) => {
                let end_time = std::time::Instant::now();
                let time_cost = (end_time - start_time).as_secs_f32();
                app.emit("upload_success", time_cost).unwrap();
                return;
            }
            Err(e) => {
                app.emit("upload_failed", e.to_string()).unwrap();
                return;
            }
        }
    }
}

#[tauri::command]
async fn download_file(app: AppHandle, object: ObjectDetail, event: Channel<ChannelBytes>) {
    let store = app.store("store.json").unwrap();
    // need to trim quotes as store won't deserialize for JSON
    let download_location = store
        .get("download_target")
        .expect("Download location not found")
        .to_string()
        .replace("\"", "");
    let download_location = PathBuf::from(download_location).join(&object.key);
    let bucket = init_bucket(&app).await.unwrap();
    app.emit("download_started", 0)
        .expect("Failed to emit event");
    let start_time = std::time::Instant::now();
    match pull_file(&bucket, &object, &download_location, event).await {
        Ok(_) => {
            let end_time = std::time::Instant::now();
            let time_cost = (end_time - start_time).as_secs_f32();
            app.emit("download_success", time_cost)
                .expect("Failed to emit event");
        }
        Err(e) => {
            app.emit("download_failed", e.to_string())
                .expect("Failed to emit event");
        }
    }
}

#[tauri::command]
fn open(path: String) {
    opener::open(&path).unwrap();
}

#[tauri::command]
async fn zip_files(app: AppHandle, files: Vec<FileDetail>) -> FileDetail {
    // find tmp dir joined with current timestamp
    let zip_path = std::env::temp_dir()
        .join(format!("remote-send-{}.zip", chrono::Local::now().timestamp()));
    let zip_file = File::create(&zip_path);
    let mut zip = zip::ZipWriter::new(zip_file.unwrap());
    for file in files {
        let options = SimpleFileOptions::default().compression_method(
            zip::CompressionMethod::Stored,
        );
        let mut file_opened = match File::open(file.path) {
            Ok(file) => file,
            Err(_) => {
                app.emit("zip_failed",
                         "Unable to open this file. Maybe you are occupying the file or have no permission to access it?".to_string()).unwrap();
                return FileDetail::default();
            }
        };
        let mut content = Vec::new();
        let zip_action = || -> Result<(), io::Error>{
            file_opened.read_to_end(&mut content)?; // note: large memory usage here
            zip.start_file(file.name, options)?;
            zip.write_all(&content)?;
            Ok(())
        };
        if let Err(e) = zip_action() {
            app.emit("zip_failed", e.to_string()).unwrap();
            return FileDetail::default();
        }

    }
    zip.finish().unwrap();
    FileDetail::from_path(&zip_path)
}

#[tauri::command]
async fn unzip_files(app: AppHandle, object: ObjectDetail) -> Vec<FileDetail> {
    let store = app.store("store.json").unwrap();
    let download_location = store
        .get("download_target")
        .expect("Download location not found")
        .to_string()
        .replace("\"", "");
    let zip_location = PathBuf::from(&download_location).join(&object.key);
    let zip_file = match File::open(&zip_location) {
        Ok(file) => file,
        Err(_) => {
            app.emit("unzip_failed",
                     "Unable to open zipped file. Maybe you are occupying the file or have no permission to access it?".to_string()).unwrap();
            return Vec::new();
        }
    };
    let mut zip = zip::ZipArchive::new(zip_file).unwrap();
    let mut files = Vec::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let out_path = PathBuf::from(&download_location).join(file.name());
        let mut outfile = match File::create(&out_path) {
            Ok(file) => file,
            Err(_) => {
                app.emit("unzip_failed", "Unable to create file when unzipping.".to_string()).unwrap();
                return Vec::new();
            }
        };
        if let Err(e) = io::copy(&mut file, &mut outfile) {
            app.emit("unzip_failed", e.to_string()).unwrap();
        };
        files.push(FileDetail::from_path(&out_path));
    }
    files
}

#[tauri::command]
fn text_to_file(text: String) -> FileDetail {
    let timestamp = chrono::Local::now().timestamp();
    let tmp_dir = std::env::temp_dir();
    let path = tmp_dir.join(format!("remote-send-{}.txt", timestamp));
    let mut file = File::create(&path).unwrap();
    file.write_all(text.as_bytes()).unwrap();
    FileDetail::from_path(&path)
}

#[tauri::command]
fn file_to_text(file: FileDetail) -> String {
    let mut file = File::open(file.path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            pick_files,
            upload_file,
            peek_latest_file,
            download_file,
            open,
            zip_files,
            unzip_files,
            text_to_file,
            file_to_text
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
