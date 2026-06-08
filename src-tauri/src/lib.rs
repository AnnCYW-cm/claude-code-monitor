pub mod session;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WindowEvent,
};

/// Push the waiting count onto the tray title.
///
/// Convention (ux-design § 2.3):
///   waiting = 0      → no title (icon only)
///   waiting = 1..=99 → render as plain digits
///   waiting ≥ 100    → "99+" (avoid menubar overflow + signal "a lot")
///
/// `None` clears the title so the tray icon shows just the template glyph.
/// The constitution forbids notifications, so this number is the **only**
/// signal the user gets that someone is waiting.
fn sync_tray_title(app: &AppHandle, waiting: usize) {
    let Some(tray) = app.tray_by_id("main") else {
        return;
    };
    let title = match waiting {
        0 => None,
        n if n >= 100 => Some("99+".to_string()),
        n => Some(n.to_string()),
    };
    let _ = tray.set_title(title.as_deref());
}

#[tauri::command]
fn list_sessions(app: AppHandle) -> Vec<session::Session> {
    let sessions = session::list();
    sync_tray_title(&app, session::waiting_count(&sessions));
    sessions
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![list_sessions])
        .on_window_event(|window, event| {
            // Intercept the red traffic-light click. We're a tray-resident app:
            // closing the window must NOT exit the process. The only exit
            // path is the tray Quit menu (S-006). Without this, clicking the
            // red button would close the window AND, since it's our only
            // window, terminate the app — surprising and undesirable.
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .icon_as_template(true)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    if event.id.as_ref() == "quit" {
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
