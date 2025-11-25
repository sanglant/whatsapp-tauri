use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Url,
};
use tauri_plugin_shell::ShellExt;

#[tauri::command]
async fn open_external_link(app: tauri::AppHandle, url: String) {
    let _ = app.shell().open(url, None);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![open_external_link])
        .setup(|app| {
            // Create tray menu
            let show_hide = MenuItem::with_id(app, "show_hide", "Show/Hide", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_hide, &quit])?;

            // Create the tray icon
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show_hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
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

            // Handle window close event - minimize to tray instead of closing
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // Prevent the window from closing
                        api.prevent_close();
                        // Hide the window instead
                        let _ = window_clone.hide();
                    }
                });

                // Inject JavaScript to proxy Notifications and handle external links
                let script = r#"
                    (function() {
                        // Wait for Tauri API to be available
                        function withTauri(fn) {
                            if (window.__TAURI__) {
                                fn();
                            } else {
                                setTimeout(() => withTauri(fn), 100);
                            }
                        }

                        withTauri(() => {
                            const { notification, core } = window.__TAURI__;

                            // Proxy Notification API
                            const OriginalNotification = window.Notification;
                            let permission = 'granted';
                            
                            window.Notification = function(title, options) {
                                notification.sendNotification({
                                    title: title,
                                    body: options ? options.body : '',
                                    icon: 'icon'
                                });
                                return {
                                    close: () => {},
                                    addEventListener: () => {},
                                    removeEventListener: () => {}
                                };
                            };
                            
                            window.Notification.permission = permission;
                            window.Notification.requestPermission = async function() {
                                permission = 'granted';
                                window.Notification.permission = permission;
                                return permission;
                            };

                            console.log('Notification API proxied via Tauri');

                            // External Link Handling via Rust Command
                            function openExternal(url) {
                                core.invoke('open_external_link', { url: url });
                            }

                            // Intercept window.open
                            const originalOpen = window.open;
                            window.open = function(url, target, features) {
                                if (url && (url.startsWith('http://') || url.startsWith('https://'))) {
                                    if (!url.includes('web.whatsapp.com')) {
                                        openExternal(url);
                                        return null;
                                    }
                                }
                                return originalOpen.call(this, url, target, features);
                            };

                            // Intercept clicks
                            document.addEventListener('click', function(e) {
                                let target = e.target;
                                while (target && target.tagName !== 'A') {
                                    target = target.parentElement;
                                }
                                if (target && target.tagName === 'A' && target.href) {
                                    const url = target.href;
                                    if ((url.startsWith('http://') || url.startsWith('https://')) && !url.includes('web.whatsapp.com')) {
                                        e.preventDefault();
                                        e.stopPropagation();
                                        openExternal(url);
                                    }
                                }
                            }, true);
                            
                            console.log('External link handler installed (Rust command)');
                        });
                    })();
                "#;

                let _ = window.eval(script);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
