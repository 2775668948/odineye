// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use base64;
use regex::Regex;
use reqwest::Client;
use std::thread;
use std::time::Duration;

fn find_lol_path() -> Option<PathBuf> {
    let candidates = vec![
        // 国服 QQ
        "C:\\Program Files (x86)\\Tencent\\League of Legends\\LeagueClient.exe",
        // 国服 Riot（通过 WeGame 安装）
        "C:\\Riot Games\\League of Legends\\LeagueClient.exe",
        // 英雄联盟手游模拟器版
        "D:\\WeGameApps\\英雄联盟\\Launcher\\Client.exe", // 自定义安装路径等也可加入
    ];

    for path_str in candidates {
        let path = PathBuf::from(path_str);
        if path.exists() {
            return Some(path);
        }
    }

    None
}

#[tauri::command]
fn start_lol() -> Result<String, String> {
    // 尝试检测 LOL 安装路径
    match find_lol_path() {
        Some(lol_path) => {
            // 启动 LOL 客户端
            match Command::new(&lol_path).spawn() {
                Ok(_) => Ok(format!("成功启动 LOL，路径为：{}", lol_path.display())),
                Err(e) => Err(format!("启动失败: {}", e)),
            }
        }
        None => Err("无法找到 League of Legends 安装路径，请手动配置".to_string()),
    }
} // 自动查找 LeagueClient.exe 的路径

#[tauri::command]
pub async fn start_auto_accept() {
    // 1. 读取 LeagueClientUx.exe 启动参数
    let output = Command::new("cmd")
        .args([
            "/C",
            "wmic PROCESS WHERE name='LeagueClientUx.exe' GET commandline",
        ])
        .output()
        .expect("failed to execute wmic");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("WMIC输出: {}", stdout);

    // 2. 正则匹配出port和token
    let port_re = Regex::new(r"--app-port=(\d+)").unwrap();
    let token_re = Regex::new(r"--remoting-auth-token=([\w-]+)").unwrap();

    let port = port_re
        .captures(&stdout)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .expect("没有找到 app-port");

    let token = token_re
        .captures(&stdout)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .expect("没有找到 remoting-auth-token");

    println!("解析成功：port={} token={}", port, token);

    // 3. 启动循环定时检测ReadyCheck
    let auth = base64::encode(format!("riot:{}", token));
    let client = Client::builder()
        .danger_accept_invalid_certs(true) // 本地自签证书
        .build()
        .unwrap();

    tauri::async_runtime::spawn(async move {
        loop {
            let res = client
                .get(format!(
                    "https://127.0.0.1:{}/lol-matchmaking/v1/ready-check",
                    port
                ))
                .header("Authorization", format!("Basic {}", auth))
                .send()
                .await;

            if let Ok(response) = res {
                if response.status().is_success() {
                    if let Ok(json) = response.json::<serde_json::Value>().await {
                        if json["state"] == "InProgress" {
                            println!("检测到对局，自动接受！");
                            let _ = client
                                .post(format!(
                                    "https://127.0.0.1:{}/lol-matchmaking/v1/ready-check/accept",
                                    port
                                ))
                                .header("Authorization", format!("Basic {}", auth))
                                .send()
                                .await;
                        }
                    }
                }
            } else {
                println!("请求失败，可能是客户端未启动");
            }

            thread::sleep(Duration::from_secs(1)); // 每1秒检查一次
        }
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_lol])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
