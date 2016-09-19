// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:

extern crate libc;

mod kernel32;

use super::Process;
use super::Problem;

fn process(entry: &kernel32::PROCESSENTRY32) -> Process {
    let mut ps = Process {
        pid: entry.pid as i32,
        ppid: entry.ppid as i32,
        name: String::from_utf16_lossy(&entry.file).trim_right_matches(0x00 as char).to_string(),
        path: None,
    };

    // Resolve Full Path.
    unsafe {
        // 0x00000400 = PROCESS_QUERY_INFORMATION
        let handle = kernel32::OpenProcess(0x00000400, 0, entry.pid);
        if (handle as i32) >= 0 {
            let mut name: [u16; 260] = [0; 260];
            let mut size = 260;
            let result = kernel32::QueryFullProcessImageNameW(handle, 0, &mut name[0], &mut size);

            // Close Process Handle.
            kernel32::CloseHandle(handle);

            // Check Result.
            if result != 0 {
                let full =
                    String::from_utf16_lossy(&name).trim_right_matches(0x00 as char).to_string();
                ps.path = Some(full);
            }
        }
    }

    return ps;
}

fn problem(code: u32) -> Problem {
    let message: String;

    // Get Error Message.
    unsafe {
        let mut buff: [u16; kernel32::MAX_PATH] = [0; kernel32::MAX_PATH];
        let size = kernel32::MAX_PATH as u32;
        let null = 0 as *mut libc::c_void;
        kernel32::FormatMessageW(kernel32::FORMAT_MESSAGE_FROM_SYSTEM,
                                 null,
                                 code,
                                 0,
                                 &mut buff[0],
                                 size,
                                 null);

        // Convert
        message = String::from_utf16_lossy(&buff).trim_right_matches(0x00 as char).to_string();
    }

    return Problem::new(message.as_str());
}

/// Create Snapshot.
fn snapshot() -> Result<Vec<kernel32::PROCESSENTRY32>, Problem> {
    let mut processes: Vec<kernel32::PROCESSENTRY32> = Vec::new();
    unsafe {
        let snapshot = kernel32::CreateToolhelp32Snapshot(kernel32::TH32CS_SNAPPROCESS, 0);
        if (snapshot as i32) < 0 {
            return Err(problem(kernel32::GetLastError()));
        }

        let mut entry = Box::new(kernel32::PROCESSENTRY32::new());

        // Get First Process.
        let mut result = kernel32::Process32FirstW(snapshot, &mut *entry);
        if result == 0 {
            kernel32::CloseHandle(snapshot);
            let error = kernel32::GetLastError();
            if error == kernel32::ERROR_NO_MORE_FILES {
                return Ok(processes);
            } else {
                return Err(problem(error));
            }
        } else {
            processes.push(*(entry.clone()));
        }

        // Load Processes.
        loop {
            result = kernel32::Process32NextW(snapshot, &mut *entry);
            if result == 0 {
                kernel32::CloseHandle(snapshot);
                let error = kernel32::GetLastError();
                if error == kernel32::ERROR_NO_MORE_FILES {
                    return Ok(processes);
                } else {
                    return Err(problem(error));
                }
            }

            processes.push(*(entry.clone()));
        }
    }
}

/// Find Windows Process by Process ID.
pub fn find(pid: u32) -> Result<Process, Problem> {
    match snapshot() {
        Ok(pss) => {
            for ps in pss {
                if ps.pid == pid {
                    return Ok(process(&ps));
                }
            }
            return Err(problem(kernel32::ERROR_NOT_FOUND));
        }
        Err(err) => {
            return Err(err);
        }
    };
}
