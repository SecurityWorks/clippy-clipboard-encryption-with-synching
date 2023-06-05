// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]

use entity::clipboard;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::{DatabaseConnection, DbErr};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};

mod connection;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    let res = insert().await;
    Ok(format!(
        "Hello, {}! You've been greeted from Rust!",
        res.unwrap()
    ))
}

async fn insert() -> Result<String, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let post = clipboard::ActiveModel {
        r#type: Set(String::from("textx")),
        content: Set(Some(String::from("Hello, World!"))),
        ..Default::default()
    };

    let post: clipboard::Model = post.insert(&db).await?;

    Ok(format!("Post created with ID: {}", post.id))
}

#[tokio::main]
async fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
