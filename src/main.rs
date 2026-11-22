#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod icon_data;
mod ui;
mod updater;

use config::AppConfig;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::window::{Theme, WindowBuilder};
use wry::{WebContext, WebViewBuilder};

#[derive(serde::Deserialize)]
struct IpcMessage {
    action: String,
    server_url: Option<String>,
    github_repo: Option<String>,
    check_updates_on_startup: Option<bool>,
    download_url: Option<String>,
    excalidraw_pen_mode: Option<bool>,
    excalidraw_palm_rejection: Option<bool>,
    excalidraw_dark_theme: Option<bool>,
    excalidraw_quick_toolbar: Option<bool>,
    ui_zoom_level: Option<f64>,
    server_id: Option<String>,
    server_name: Option<String>,
    onlyoffice_url: Option<String>,
    media_player_enabled: Option<bool>,
    office_viewer_enabled: Option<bool>,
    filename: Option<String>,
    file_base64: Option<String>,
    file_url: Option<String>,
}

#[derive(Debug, Clone)]
enum UserEvent {
    UpdateAvailable(updater::ReleaseInfo),
    UpdateCheckFinished(String),
}

fn main() {
    if let Err(e) = run() {
        let _ = std::fs::write("startup_error.log", format!("Error: {:?}", e));
        eprintln!("Fatal error: {:?}", e);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load();
    let current_version = env!("CARGO_PKG_VERSION");

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let icon = tao::window::Icon::from_rgba(icon_data::ICON_RGBA_32.to_vec(), 32, 32).ok();

    let window = WindowBuilder::new()
        .with_title("Docmost")
        .with_window_icon(icon)
        .with_inner_size(LogicalSize::new(
            config.window_width as f64,
            config.window_height as f64,
        ))
        .with_theme(Some(Theme::Dark))
        .build(&event_loop)?;

    // Windows 10/11 DWM Immersive Dark Titlebar aktivieren
    #[cfg(windows)]
    unsafe {
        use tao::platform::windows::WindowExtWindows;
        use windows_sys::Win32::Graphics::Dwm::{
            DwmSetWindowAttribute, DWMWA_USE_IMMERSIVE_DARK_MODE,
        };
        let hwnd = window.hwnd() as isize;
        let dark_mode: i32 = 1;
        let _ = DwmSetWindowAttribute(
            hwnd as _,
            DWMWA_USE_IMMERSIVE_DARK_MODE as _,
            &dark_mode as *const _ as _,
            std::mem::size_of::<i32>() as u32,
        );
    }

    let shared_config = Arc::new(Mutex::new(config.clone()));
    let shared_config_for_ipc = Arc::clone(&shared_config);
    let proxy_for_ipc = proxy.clone();

    let pending_update: Arc<Mutex<Option<updater::ReleaseInfo>>> = Arc::new(Mutex::new(None));
    let pending_update_for_ipc = Arc::clone(&pending_update);
    let pending_update_for_events = Arc::clone(&pending_update);

    // Lokaler Ordner für persistente Cookies, Sessions und Caches
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("DocmostApp")
        .join("WebView2Data");
    let _ = std::fs::create_dir_all(&data_dir);

    let mut web_context = WebContext::new(Some(data_dir));
    let injection_script = ui::get_injection_script(&config, current_version);

    let mut builder = WebViewBuilder::new_with_web_context(&mut web_context)
        .with_devtools(true)
        .with_initialization_script(&injection_script);

    if config.server_url.trim().is_empty() {
        builder = builder.with_html(ui::get_offline_or_setup_html(&config.server_url, current_version, None));
    } else {
        builder = builder.with_url(&config.server_url);
    }

    let webview = builder
        .with_ipc_handler(move |req: wry::http::Request<String>| {
            let msg = req.body();
            if let Ok(parsed) = serde_json::from_str::<IpcMessage>(msg) {
                match parsed.action.as_str() {
                    "save_settings" => {
                        let mut cfg = shared_config_for_ipc.lock().unwrap();
                        if let Some(url) = parsed.server_url {
                            if !url.trim().is_empty() {
                                cfg.server_url = url;
                            }
                        }
                        if let Some(repo) = parsed.github_repo {
                            if !repo.trim().is_empty() {
                                cfg.github_repo = repo;
                            }
                        }
                        if let Some(auto_up) = parsed.check_updates_on_startup {
                            cfg.check_updates_on_startup = auto_up;
                        }
                        if let Some(pm) = parsed.excalidraw_pen_mode {
                            cfg.excalidraw_pen_mode = pm;
                        }
                        if let Some(pr) = parsed.excalidraw_palm_rejection {
                            cfg.excalidraw_palm_rejection = pr;
                        }
                        if let Some(dt) = parsed.excalidraw_dark_theme {
                            cfg.excalidraw_dark_theme = dt;
                        }
                        if let Some(qt) = parsed.excalidraw_quick_toolbar {
                            cfg.excalidraw_quick_toolbar = qt;
                        }
                        if let Some(zoom) = parsed.ui_zoom_level {
                            cfg.ui_zoom_level = zoom;
                        }
                        if let Some(oo) = parsed.onlyoffice_url {
                            cfg.onlyoffice_url = oo.trim().to_string();
                        }
                        if let Some(mp) = parsed.media_player_enabled {
                            cfg.media_player_enabled = mp;
                        }
                        if let Some(ov) = parsed.office_viewer_enabled {
                            cfg.office_viewer_enabled = ov;
                        }
                        let _ = cfg.save();
                    }
                    "open_temp_file" => {
                        if let (Some(name), Some(b64)) = (parsed.filename, parsed.file_base64) {
                            thread::spawn(move || {
                                use base64::Engine;
                                let engine = base64::engine::general_purpose::STANDARD;
                                if let Ok(bytes) = engine.decode(&b64) {
                                    let temp_dir = std::env::temp_dir().join("DocmostApp").join("Files");
                                    let _ = std::fs::create_dir_all(&temp_dir);
                                    let clean_name = name.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
                                    let target = temp_dir.join(&clean_name);
                                    if std::fs::write(&target, bytes).is_ok() {
                                        let _ = open::that(&target);
                                    }
                                }
                            });
                        }
                    }
                    "open_file_external" => {
                        if let Some(url) = parsed.file_url {
                            thread::spawn(move || {
                                let _ = open::that(url);
                            });
                        }
                    }
                    "switch_server" => {
                        let mut cfg = shared_config_for_ipc.lock().unwrap();
                        if let Some(id) = parsed.server_id {
                            if cfg.set_active_server(&id) {
                                let _ = cfg.save();
                            }
                        }
                    }
                    "add_server" => {
                        let mut cfg = shared_config_for_ipc.lock().unwrap();
                        if let (Some(name), Some(url)) = (parsed.server_name, parsed.server_url) {
                            let mut clean_url = url.trim().to_string();
                            if !clean_url.starts_with("http://") && !clean_url.starts_with("https://") {
                                clean_url = format!("https://{}", clean_url);
                            }
                            let id = format!("srv_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
                            let clean_name = if name.trim().is_empty() {
                                clean_url.replace("https://", "").replace("http://", "")
                            } else {
                                name.trim().to_string()
                            };
                            let entry = config::ServerEntry {
                                id: id.clone(),
                                name: clean_name,
                                url: clean_url.clone(),
                            };
                            cfg.servers.push(entry);
                            cfg.active_server_id = Some(id);
                            cfg.server_url = clean_url;
                            let _ = cfg.save();
                        }
                    }
                    "delete_server" => {
                        let mut cfg = shared_config_for_ipc.lock().unwrap();
                        if let Some(id) = parsed.server_id {
                            if cfg.servers.len() > 1 {
                                cfg.servers.retain(|s| s.id != id);
                                if cfg.active_server_id.as_deref() == Some(&id) {
                                    let first_opt = cfg.servers.first().cloned();
                                    if let Some(first) = first_opt {
                                        cfg.active_server_id = Some(first.id);
                                        cfg.server_url = first.url;
                                    }
                                }
                                let _ = cfg.save();
                            }
                        }
                    }
                    "check_pending_update" => {
                        let maybe_info = pending_update_for_ipc.lock().unwrap().clone();
                        if let Some(info) = maybe_info {
                            let _ = proxy_for_ipc.send_event(UserEvent::UpdateAvailable(info));
                        }
                    }
                    "check_updates" => {
                        let cfg = shared_config_for_ipc.lock().unwrap().clone();
                        let p = proxy_for_ipc.clone();
                        let cur_ver = env!("CARGO_PKG_VERSION").to_string();
                        let pend = Arc::clone(&pending_update_for_ipc);
                        thread::spawn(move || {
                            match updater::check_for_updates(&cfg.github_repo, &cur_ver) {
                                Ok(Some(info)) => {
                                    *pend.lock().unwrap() = Some(info.clone());
                                    let _ = p.send_event(UserEvent::UpdateAvailable(info));
                                }
                                Ok(None) => {
                                    let _ = p.send_event(UserEvent::UpdateCheckFinished(format!("Du hast bereits die aktuellste Version (v{})!", cur_ver)));
                                }
                                Err(err) => {
                                    let _ = p.send_event(UserEvent::UpdateCheckFinished(format!("Fehler: {}", err)));
                                }
                            }
                        });
                    }
                    "download_and_install_update" => {
                        if let Some(url) = parsed.download_url {
                            thread::spawn(move || {
                                let _ = updater::download_and_run_installer(&url);
                            });
                        }
                    }
                    "open_external" => {
                        if let Some(url) = parsed.server_url {
                            let _ = open::that(url);
                        }
                    }
                    _ => {}
                }
            }
        })
        .build(&window)?;

    // Hintergrund-Thread: Automatischer Update-Check beim Programmstart
    if config.check_updates_on_startup {
        let repo = config.github_repo.clone();
        let cur_ver = current_version.to_string();
        let p = proxy.clone();
        let pend = Arc::clone(&pending_update);
        thread::spawn(move || {
            // Kurze Pause von 2 Sekunden für reibungslosen UI-Start
            thread::sleep(Duration::from_secs(2));
            if let Ok(Some(info)) = updater::check_for_updates(&repo, &cur_ver) {
                *pend.lock().unwrap() = Some(info.clone());
                let _ = p.send_event(UserEvent::UpdateAvailable(info.clone()));

                // Nochmal nach 3 Sekunden senden falls die Webseite gerade erst geladen/weitergeleitet hat
                thread::sleep(Duration::from_secs(3));
                let _ = p.send_event(UserEvent::UpdateAvailable(info));
            }
        });
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(user_event) => match user_event {
                UserEvent::UpdateAvailable(info) => {
                    *pending_update_for_events.lock().unwrap() = Some(info.clone());
                    if let Ok(json) = serde_json::to_string(&info) {
                        let script = format!(
                            "window.__pendingUpdateInfo = {json}; if (typeof window.onUpdateAvailable === 'function') {{ window.onUpdateAvailable({json}); }}",
                            json = json
                        );
                        let _ = webview.evaluate_script(&script);
                    }
                }
                UserEvent::UpdateCheckFinished(status) => {
                    let script = format!(
                        "const el = document.getElementById('update-status-text'); if(el) {{ el.innerText = '{}'; el.style.color = '#10b981'; }}",
                        status.replace('\'', "\\'")
                    );
                    let _ = webview.evaluate_script(&script);
                }
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    let mut cfg = shared_config.lock().unwrap();
                    let size = window.inner_size();
                    cfg.window_width = size.width;
                    cfg.window_height = size.height;
                    let _ = cfg.save();
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    });
}
