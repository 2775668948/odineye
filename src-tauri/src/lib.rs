use fs_extra::dir;
use image::DynamicImage;
use image::{Rgb, RgbImage};
use scrap::{Capturer, Display};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use tauri_plugin_global_shortcut::{Builder, Shortcut, ShortcutEvent};
use xcap::Monitor;

use tauri::AppHandle;

fn normalized(filename: String) -> String {
    filename.replace(['|', '\\', ':', '/'], "")
}

// 获取lol的配置信息
pub async fn get_lol_token() {
    ("").to_string();
}

// 获取玩家id

// 屏幕全屏截图
pub async fn screen_shoct() {
    let monitors = Monitor::all().unwrap();

    dir::create_all("target/monitors", true).unwrap();

    for monitor in monitors {
        let image = monitor.capture_image().unwrap();

        let start = Instant::now();
        image.save("target/monitors/monitor-{}.png").unwrap();
        println!("运行耗时: {:?}", start.elapsed());
    }

    println!("屏幕截图成功")
}

