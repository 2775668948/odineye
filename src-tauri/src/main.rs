mod lcu;

use tauri::AppHandle;
use tauri::Manager;
use tauri::Window;

use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use base64::{engine::general_purpose, Engine as _};
use reqwest::{header, Client};
use std::error::Error;

use odineye_lib::{get_lol_token, screen_shoct};

use std::{thread, time};

use irelia::{rest::LcuClient, RequestClient};
use serde_json::Value;
use tauri_plugin_http;

// 全局token和port
static mut LOL_TOKEN: String = String::new();
static mut LOL_PORT: u16 = 0;

#[tauri::command]
fn closegmwindow(window: Window) -> Result<(), String> {
    println!("准备隐藏窗口: {}", window.label()); // 打印窗口标签名
    match window.hide() {
        Ok(_) => {
            println!("窗口隐藏成功！");
            Ok(())
        }
        Err(e) => {
            println!("窗口隐藏失败: {}", e);
            Err(e.to_string())
        }
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
    println!("okkkk");
}

#[tauri::command]
async fn closegmwindow2(window: Window) -> Result<(), String> {
    println!("准备设置鼠标穿透: {}", window.label()); // 打印窗口标签名
    match window.set_ignore_cursor_events(true) {
        Ok(_) => {
            println!("鼠标穿透设置成功！");
            screen_shoct().await;
            Ok(())
        }
        Err(e) => {
            println!("鼠标穿透设置失败: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn showgmwindow(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("gamestart") {
        window.show().map_err(|e| e.to_string())?; // 隐藏窗口
        println!("已重新展示窗口: gamestart");
        Ok(())
    } else {
        Err("找不到窗口: gamestart".to_string())
    }
}

#[tauri::command]
fn handle_tab_key_pressed() -> String {
    // 处理 Tab 键按下后的逻辑
    "快捷键被按下了".to_string()
}

fn startautoaccept() -> String {
    // 处理 Tab 键按下后的逻辑
    "后端自动开启对局已经Ok...".to_string()
}

fn getlolinfo() -> String {
    // 处理 Tab 键按下后的逻辑
    "获取port和token...".to_string()
}

async fn init_config() {
    println!("执行异步初始化逻辑");
    // 异步代码，如网络请求、文件 IO 等
    // autoaccept().await;
    // get_lol_token().await;
    // 调用同步函数，直接写，不需要 .await
    match lcu::get_riot_token_and_port() {
        Ok((token, port)) => {
            println!("获取到 token: {}, port: {}", token, port);
            unsafe {
                LOL_TOKEN = token;
                LOL_PORT = port;
            }
        }
        Err(e) => {
            eprintln!("获取失败: {}", e);
        }
    }
}

// async fn autoaccept() -> Result<String, String> {
//     println!("开启自动接受对局功能");
//     let port = 63304;
//     let token = "Y9X1aNkE6yzyZhcX9znzQw";
//     let endpoint = "/lol-matchmaking/v1/ready-check/accept";
//     loop {
//         println!("检查是否开启对局...");
//         match lcu_post_request(port, token, endpoint).await {
//             Ok(res) => {
//                 println!("自动接受对局成功: {}", res);
//             }
//             Err(err) => {
//                 eprintln!("自动接受对局失败: {}", err);
//             }
//         }
//         thread::sleep(time::Duration::from_secs(1)); // 每5秒检查一次准备状态
//     }
// }

#[tauri::command]
fn enable_click_through(window: tauri::Window) {}

#[tauri::command]
// 返回port和token给前端
fn return_port_and_token() -> Result<(String, u16), String> {
    println!("前端获取token...");
    unsafe {
        if LOL_TOKEN.is_empty() || LOL_PORT == 0 {
            return Err("未获取到有效的 token 或 port".to_string());
        }
        Ok((LOL_TOKEN.clone(), LOL_PORT))
    }
}

#[tauri::command]
async fn lcu_post_request(endpoint: String) -> Result<String, String> {
    let port = unsafe { LOL_PORT };
    let token = unsafe { LOL_TOKEN.clone() };
    let url = format!("https://127.0.0.1:{}{}", port, endpoint);

    let auth_string = format!("riot:{}", token);
    let auth_encoded = general_purpose::STANDARD.encode(auth_string);
    let auth_header_value = format!("Basic {}", auth_encoded);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT, "application/json".parse().unwrap());
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(header::AUTHORIZATION, auth_header_value.parse().unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;
    println!("请求的URL-->: {}", url);
    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;
    let result = response.text().await.map_err(|e| e.to_string())?;
    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            tauri::async_runtime::spawn(async {
                init_config().await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            closegmwindow,
            showgmwindow,
            handle_tab_key_pressed,
            enable_click_through,
            closegmwindow2,
            return_port_and_token,
            lcu_post_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
