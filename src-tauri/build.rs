use std::{env, fs::{create_dir_all, remove_file, remove_dir_all, read_dir, rename, File, OpenOptions}, io::Write, path::Path};
use tokio::runtime::Runtime;
use reqwest;
use tauri_build;
use zip::ZipArchive;

const YTDLP_URL: &str = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe";
const FFMPEG_URL: &str = "https://github.com/BtbN/FFmpeg-Builds/releases/latest/download/ffmpeg-master-latest-win64-gpl-shared.zip";

fn main() {
    let temp_folder = env::current_dir().unwrap()
        .parent().unwrap()
        .join("temp");

    if !temp_folder.exists() {
        create_dir_all(&temp_folder).unwrap();
    }

    Some(temp_folder.join("yt-dlp.exe"))
        .take_if(|path| !path.exists())
        .map(|path| {
            Runtime::new().unwrap().block_on(
                download_file(YTDLP_URL, &path)
            );
        });

    Some(temp_folder.join("ffmpeg"))
        .take_if(|path| !path.join("ffmpeg.exe").exists())
        .map(|path| {
            Runtime::new().unwrap().block_on(
                download_and_extract_archive(FFMPEG_URL, &path)
            );

            let shit_dir = path.join("ffmpeg-master-latest-win64-gpl-shared");
    
            for entry in read_dir(shit_dir.join("bin")).unwrap() {
                let entry = entry.unwrap();
                let entry_path = entry.path();
                let target_path = path.join(entry.file_name());

                rename(entry_path, target_path).unwrap();
            }

            remove_dir_all(shit_dir).unwrap();
        });

    Some(temp_folder.join("cookies.txt"))
        .take_if(|path| !path.exists())
        .map(|path| File::create(path));

    Some(temp_folder.join("output"))
        .take_if(|path| !path.exists())
        .map(|path| create_dir_all(path));

    tauri_build::build();
}

async fn download_file(url: &str, output_path: &Path) {
    let content = reqwest::get(url).await.unwrap().bytes().await.unwrap();
    let mut file = File::create(output_path).unwrap();
    file.write_all(&content).unwrap();
}

async fn download_and_extract_archive(url: &str, output_path: &Path) {
    if !output_path.exists() {
        create_dir_all(&output_path).unwrap();
    }

    let file_path = output_path.join("archive.zip");
    download_file(url, &file_path).await;

    let file = OpenOptions::new().read(true).open(&file_path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    archive.extract(output_path).unwrap();
    remove_file(file_path).unwrap();
}