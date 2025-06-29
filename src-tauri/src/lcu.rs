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
    let re_token = Regex::new(r#"--remoting-auth-token=("[^"]+"|[^\s"]+)"#).ok()?;
    let re_port = Regex::new(r#"--app-port=("[^"]+"|[^\s"]+)"#).ok()?;

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
