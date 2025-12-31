use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

const CUSTOM_CSS: &str = "
/* WhatsApp Lite Custom CSS */
/* Hide Communities, Channels, Meta AI (Keep Chats, Status, Settings, Profile) */
div[title='Communities'], span[title='Communities'], div[aria-label='Communities'], span[aria-label='Communities'],
div[title='Channels'], span[title='Channels'], div[aria-label='Channels'], span[aria-label='Channels'],
div[title='Meta AI'], span[title='Meta AI'], div[aria-label='Meta AI'], span[aria-label='Meta AI'] {
    display: none !important;
}

/* Hide 'Get the app' banners if any */
div[data-testid='intro-title'], div[data-testid='intro-text'] {
    display: none !important;
}

/* Cool Scrollbar */
::-webkit-scrollbar {
    width: 6px !important;
    height: 6px !important;
}
::-webkit-scrollbar-track {
    background: transparent !important;
}
::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2) !important;
    border-radius: 3px !important;
}
::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3) !important;
}
";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .plugin(tauri_plugin_global_shortcut::Builder::new().build()) // Commented out until configured
        .setup(|app| {
            let _handle = app.handle();

            // System Tray
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show/Hide", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => app.exit(0),
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
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

            // CSS Injection
            if let Some(window) = app.get_webview_window("main") {
                let script = format!(
                    "
                    const init = () => {{
                        const css = `{}`;
                        const style = document.createElement('style');
                        style.textContent = css;
                        document.head.append(style);

                        const observer = new MutationObserver((mutations) => {{
                            mutations.forEach((mutation) => {{
                                mutation.addedNodes.forEach((node) => {{
                                    if (node.nodeType === 1) {{
                                        const text = node.innerText;
                                        if (text && (
                                            text.includes('Get WhatsApp for Windows') ||
                                            text.includes('See more chat history') ||
                                            text.includes('Download WhatsApp for Windows') ||
                                            text.includes('Make calls, share your screen')
                                        )) {{
                                            node.style.display = 'none';
                                        }}
                                        const banners = node.querySelectorAll('div, span, a');
                                        banners.forEach(b => {{
                                            const t = b.innerText;
                                            if (t && (
                                                t.includes('Get WhatsApp for Windows') ||
                                                t.includes('See more chat history') ||
                                                t.includes('Download WhatsApp for Windows') ||
                                                t.includes('Make calls, share your screen')
                                            )) {{
                                                const container = b.closest('div[role=\"button\"]') || b.closest('div._aigv') || b.closest('div._aigw') || b;
                                                if (container) container.style.display = 'none';
                                            }}
                                        }});
                                    }}
                                }});
                            }});
                        }});

                        if (document.body) {{
                            observer.observe(document.body, {{ childList: true, subtree: true }});
                        }}
                    }};

                    if (document.readyState === 'loading') {{
                        window.addEventListener('DOMContentLoaded', init);
                    }} else {{
                        init();
                    }}
                    ",
                    CUSTOM_CSS
                );
                window.eval(&script)?;
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
