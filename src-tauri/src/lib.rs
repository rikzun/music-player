use std::{env, process::{Command, Stdio}, sync::Arc, thread};
use tauri::{AppHandle, Emitter};
use std::io::{BufRead, BufReader};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(app: AppHandle, value: String) {
    let temp_folder = env::current_dir().unwrap()
        .parent().unwrap()
        .join("temp");

    let output_folder = temp_folder.join("output").join("%(title)s.%(ext)s");
    let cookies = temp_folder.join("cookies.txt");
    let ffmpeg = temp_folder.join("ffmpeg");
    let ytdlp = temp_folder.join("yt-dlp.exe");

    let mut cmd = Command::new(ytdlp);
    cmd.arg(format!("--cookies={}", cookies.display()));
    cmd.arg("-x");
    cmd.arg("--audio-format=mp3");
    cmd.arg(format!("--output={}", output_folder.display()));
    cmd.arg(format!("--ffmpeg-location={}", ffmpeg.display()));
    cmd.arg(value);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("aboba");

    app.emit("command", command_to_string(&cmd)).unwrap();

    let stdout = child.stdout.take().expect("Не удалось получить stdout");
    let stderr = child.stderr.take().expect("Не удалось получить stderr");

    let apppp = Arc::new(app); // Подставь свой `app`
    let app_clone1 = Arc::clone(&apppp);
    let app_clone2 = Arc::clone(&apppp);
    
    let stdout_thread = thread::spawn(move || read_stream(app_clone1, stdout, "stdout-event"));
    let stderr_thread = thread::spawn(move || read_stream(app_clone2, stderr, "stderr-event"));

    stdout_thread.join().expect("Ошибка при ожидании stdout");
    stderr_thread.join().expect("Ошибка при ожидании stderr");

    child.wait().expect("Ошибка ожидания процесса");
}

fn command_to_string(cmd: &Command) -> String {
    let program = cmd.get_program().to_string_lossy();
    let args: Vec<String> = cmd.get_args().map(|arg| arg.to_string_lossy().into_owned()).collect();
    format!("{} {}", program, args.join(" "))
}

fn read_stream(app: Arc<AppHandle>, stream: impl std::io::Read, event: &str) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(text) => {
                app.emit(event, format!("> {}", text)).unwrap();
            }
            Err(e) => eprintln!("Ошибка чтения {}", e),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
