use regex::Regex;

use base64::{engine::general_purpose, Engine as _};
use reqwest::{header, Client};
use std::error::Error;
use std::{
    ffi::{c_void, OsString},
    mem,
    os::windows::ffi::OsStringExt,
    ptr, slice,
};
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::ToolHelp::*,
        Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION},
    },
};

const ProcessCommandLineInformation: u32 = 60;

#[repr(C)]
struct UNICODE_STRING {
    Length: u16,
    MaximumLength: u16,
    Buffer: *const u16,
}

#[link(name = "ntdll")]
extern "system" {
    fn NtQueryInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: u32,
        ProcessInformation: *mut c_void,
        ProcessInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> i32;
}

fn get_pid_by_name(name: &str) -> Vec<u32> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap();
        let mut entry = PROCESSENTRY32W {
            dwSize: mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        let mut pids = Vec::new();
        if Process32FirstW(snapshot, &mut entry).as_bool() {
            loop {
                let exe_name = {
                    let len = entry
                        .szExeFile
                        .iter()
                        .position(|&c| c == 0)
                        .unwrap_or(entry.szExeFile.len());
                    OsString::from_wide(&entry.szExeFile[..len])
                        .to_string_lossy()
                        .to_string()
                };
                if exe_name.to_lowercase().contains(&name.to_lowercase()) {
                    pids.push(entry.th32ProcessID);
                }
                if !Process32NextW(snapshot, &mut entry).as_bool() {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
        pids
    }
}

fn get_process_command_line(pid: u32) -> windows::core::Result<String> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)?;
        if handle.0 == 0 {
            return Err(windows::core::Error::from_win32());
        }

        let mut return_length = 0u32;

        NtQueryInformationProcess(
            handle,
            ProcessCommandLineInformation,
            ptr::null_mut(),
            0,
            &mut return_length,
        );

        if return_length == 0 {
            CloseHandle(handle);
            return Err(windows::core::Error::from_win32());
        }

        let mut buffer = vec![0u8; return_length as usize];

        let status = NtQueryInformationProcess(
            handle,
            ProcessCommandLineInformation,
            buffer.as_mut_ptr() as *mut c_void,
            return_length,
            &mut return_length,
        );

        if status != 0 {
            CloseHandle(handle);
            return Err(windows::core::Error::from_win32());
        }

        let ustr = &*(buffer.as_ptr() as *const UNICODE_STRING);
        let slice = slice::from_raw_parts(ustr.Buffer, (ustr.Length / 2) as usize);
        let cmd_line = String::from_utf16_lossy(slice);

        CloseHandle(handle);
        Ok(cmd_line)
    }
}

fn extract_token_and_port(cmdline: &str) -> Option<(String, u16)> {
    let re_token = Regex::new(r#"--riotclient-auth-token=("[^"]+"|[^\s"]+)"#).ok()?;
    let re_port = Regex::new(r#"--riotclient-app-port=("[^"]+"|[^\s"]+)"#).ok()?;

    let token = re_token
        .captures(cmdline)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().trim_matches('"').to_string()));

    let port = re_port.captures(cmdline).and_then(|cap| {
        cap.get(1)
            .and_then(|m| m.as_str().trim_matches('"').parse::<u16>().ok())
    });

    match (token, port) {
        (Some(t), Some(p)) => Some((t, p)),
        _ => None,
    }
}

pub fn get_riot_token_and_port() -> Result<(String, u16), String> {
    let pids = get_pid_by_name("LeagueClientUx.exe");
    if pids.is_empty() {
        return Err("未找到 LeagueClientUx.exe 进程".into());
    }

    for pid in pids {
        if let Ok(cmdline) = get_process_command_line(pid) {
            if let Some((token, port)) = extract_token_and_port(&cmdline) {
                return Ok((token, port));
            }
        }
    }

    Err("未能提取 token 或 port".into())
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
