use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_notification::NotificationExt;
use tauri::UserAttentionType;
use tauri_plugin_opener::OpenerExt;

const LINK_REDIRECT_SCRIPT: &str = r#"
(function (){
    console.log('Link observer started')

    document.addEventListener('click', (e) => {
        const anchor = e.target.closest('a');
        if(!anchor) return;

        if (anchor.getAttribute("role") == "link" && anchor.target === "_blank") {
            e.preventDefault();

            if (window.__TAURI__ && window.__TAURI__.core) {
                window.__TAURI__.core.invoke('open_link', { link: anchor.getAttribute("href") })
                    .catch(err => console.error("Failed while opening link: ", err));
            } else {
                console.error("Tauri API not found!")
            }
        }
    })
})()
"#;

const INJECT_SCRIPT: &str = r#"
(function() {
    if (window.isObserverRunning) return;
    window.isObserverRunning = true;

    console.log("Observer Script Started.");

    if (!window.__TAURI__) {
        console.error("window.__TAURI__ does not exists");
    } else {
        console.log("✅ notifications observer active");
    }

    let lastSentTitle = "";
    
    function notifyRust(title) {
        if (window.__TAURI__ && window.__TAURI__.core) {
             window.__TAURI__.core.invoke('notify_command', { title: title })
                .catch(err => console.error("backend e rror:", err));
        } else {
             console.error('tauri api error')
        }
    }

    setInterval(() => {
        const currentTitle = document.title;

        const isNotificationFormat = /^\(\d+\)/.test(currentTitle);

        if (isNotificationFormat) {
            if (currentTitle !== lastSentTitle) {
                lastSentTitle = currentTitle;
                
                notifyRust(currentTitle);
            }
        }
    }, 1000);
})();
"#;

#[tauri::command]
fn notify_command(app: AppHandle, title: String) {
    let _ = app.notification()
        .builder()
        .title("New message!")
        .body(&title)
        .show();
    
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.request_user_attention(Some(UserAttentionType::Critical));
    }
}

#[tauri::command]
fn open_link(app: AppHandle, link: String) {
    app.opener().open_path(link, None::<&str>);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![notify_command, open_link])
        .setup(|app| {
            let win = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://www.messenger.com/".parse().unwrap())
            )
            .title("Messenger Wrapper")
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/143.0.0.0 Safari/537.36 Edg/143.0.3650.96")
            .inner_size(800.0, 600.0)
            .initialization_script(INJECT_SCRIPT)
            .initialization_script(LINK_REDIRECT_SCRIPT)
            .disable_drag_drop_handler()
            .build()?;

            win.set_focus()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
