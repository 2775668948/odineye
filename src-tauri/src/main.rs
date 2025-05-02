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
}

async fn lcu_post_request(
    port: u16,
    token: &str,
    endpoint: &str,
) -> Result<String, Box<dyn Error>> {
    // 构造 URL
    let url = format!("https://127.0.0.1:{}{}", port, endpoint);

    // 构造 Base64 授权头
    let auth_string = format!("riot:{}", token);
    let auth_encoded = general_purpose::STANDARD.encode(auth_string);
    let auth_header_value = format!("Basic {}", auth_encoded);

    // 构造 Headers
    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT, "application/json".parse()?);
    headers.insert(header::CONTENT_TYPE, "application/json".parse()?);
    headers.insert(header::AUTHORIZATION, auth_header_value.parse()?);

    // 创建 Client，允许自签名证书
    let client = Client::builder()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .build()?;

    // 发起 GET 请求
    let response = client.get(&url).send().await?;

    // 返回文本内容
    let result = response.text().await?;
    Ok(result)
}

async fn autoaccept() -> Result<String, String> {
    println!("开启自动接受对局功能");
    let port = 63304;
    let token = "Y9X1aNkE6yzyZhcX9znzQw";
    let endpoint = "/lol-matchmaking/v1/ready-check/accept";
    loop {
        println!("检查是否开启对局...");
        match lcu_post_request(port, token, endpoint).await {
            Ok(res) => {
                println!("自动接受对局成功: {}", res);
            }
            Err(err) => {
                eprintln!("自动接受对局失败: {}", err);
            }
        }
        thread::sleep(time::Duration::from_secs(1)); // 每5秒检查一次准备状态
    }
}

#[tauri::command]
fn enable_click_through(window: tauri::Window) {}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            println!("Tauri 后端初始化逻辑运行！");
            // 这里可以做比如路径缓存加载、配置初始化等
            tauri::async_runtime::spawn(async {
                init_config().await;
            });
            Ok(())
        })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            closegmwindow,
            showgmwindow,
            handle_tab_key_pressed,
            enable_click_through,
            closegmwindow2
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
