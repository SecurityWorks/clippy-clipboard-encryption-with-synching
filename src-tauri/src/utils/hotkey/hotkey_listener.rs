use crate::{
    service::{hotkey::get_all_hotkeys_db, window::toggle_main_window},
    types::types::Key,
    utils::setup::{APP, GLOBAL_EVENTS, HOTKEYS, HOTKEY_MANAGER, HOTKEY_STOP_TX},
};
use core::time::Duration;
use global_hotkey::hotkey::HotKey;
use global_hotkey::GlobalHotKeyEvent;
use tauri::Manager;
use tokio::sync::oneshot;

pub fn init_hotkey_listener(i: i32) -> () {
    println!("init_hotkey_listener");

    tauri::async_runtime::spawn(async {
        unregister_hotkeys(true);
        let _ = upsert_hotkeys_in_store().await;
        register_hotkeys()
    });

    // If there's an existing sender, send a stop signal to the previous task
    if let Some(sender) = HOTKEY_STOP_TX.get().unwrap().lock().unwrap().take() {
        let _ = sender.send(());
    }

    let (new_stop_tx, mut stop_rx) = oneshot::channel();
    *HOTKEY_STOP_TX.get().unwrap().lock().unwrap() = Some(new_stop_tx);
    let receiver = GlobalHotKeyEvent::receiver();
    tauri::async_runtime::spawn(async move {
        loop {
            println!("{}", i);
            if let Ok(event) = receiver.try_recv() {
                let hotkeys = HOTKEYS.get().unwrap().lock().unwrap();

                if let Some(hotkey) = hotkeys.get(&event.id) {
                    println!("Hotkey Pressed: {:?}", hotkey);
                    toggle_main_window();
                }
            }

            if stop_rx.try_recv().is_ok() {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    });
}

pub fn register_hotkeys() {
    let hotkeys_store: std::sync::MutexGuard<'_, std::collections::HashMap<u32, Key>> =
        HOTKEYS.get().unwrap().lock().unwrap();
    let hotkey_manager = HOTKEY_MANAGER.get().unwrap();
    let window = APP.get().unwrap().get_window("main").unwrap();

    for (_, hotkey) in hotkeys_store.iter() {
        if window.is_visible().unwrap() {
            println!("register_hotkeys");
            hotkey_manager.register(hotkey.hotkey.clone()).unwrap();
        } else if hotkey.global {
            let key = hotkey_manager.register(hotkey.hotkey.clone());
            if key.is_err() {
                hotkey_manager.unregister(hotkey.hotkey.clone()).unwrap();
                hotkey_manager.register(hotkey.hotkey.clone()).unwrap();
            } else {
                key.unwrap();
            }
        }
    }
}

pub fn unregister_hotkeys(all: bool) {
    let hotkeys_store: std::sync::MutexGuard<'_, std::collections::HashMap<u32, Key>> =
        HOTKEYS.get().unwrap().lock().unwrap();
    let hotkey_manager = HOTKEY_MANAGER.get().unwrap();

    for (_, hotkey) in hotkeys_store.iter() {
        if all {
            hotkey_manager.unregister(hotkey.hotkey.clone()).unwrap();
        } else if !hotkey.global {
            hotkey_manager.unregister(hotkey.hotkey.clone()).unwrap();
        }
    }
}

pub async fn upsert_hotkeys_in_store() -> anyhow::Result<()> {
    let hotkeys = get_all_hotkeys_db().await?;
    let mut hotkey_store = HOTKEYS.get().unwrap().lock().unwrap();

    for hotkey in hotkeys {
        let hotkey_str: String = parse_shortcut(
            hotkey.ctrl,
            hotkey.alt,
            hotkey.shift,
            &hotkey.key.to_lowercase(),
        );

        let key: HotKey = hotkey_str.parse()?;

        let global = GLOBAL_EVENTS.contains(&hotkey.event.as_str());

        let key = Key {
            id: key.id(),
            global,
            event: hotkey.event,
            key_str: hotkey_str,
            ctrl: hotkey.ctrl,
            alt: hotkey.alt,
            shift: hotkey.shift,
            key: hotkey.key,
            hotkey: key,
        };

        if hotkey_store.get(&key.id).is_some() {
            let _ = hotkey_store.remove(&key.id);
        }

        hotkey_store.insert(key.id, key);
    }

    Ok(())
}

pub fn parse_shortcut(ctrl: bool, alt: bool, shift: bool, key: &str) -> String {
    let mut modifiers = Vec::new();
    if ctrl {
        modifiers.push("Control");
    }
    if alt {
        modifiers.push("Alt");
    }
    if shift {
        modifiers.push("Shift");
    }

    format!(
        "{}{}Key{}",
        modifiers.join("+"),
        if !modifiers.is_empty() { "+" } else { "" },
        key.to_uppercase()
    )
}
