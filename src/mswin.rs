// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:
#[cfg(windows)]

#[allow(non_camel_case_types)]
mod kernel32 {
    extern crate libc;

    pub type HANDLE  = *mut libc::c_void;
    pub type HMODULE = HANDLE;

    #[repr(C)]
    pub struct PROCESSENTRY32 {
        pub size: u32,
        pub usage: u32,
        pub pid: u32,
        pub heap: libc::uintptr_t,
        pub module: u32,
        pub threads: u32,
        pub ppid: u32,
        pub priority: i32,
        pub flags: u32,
        pub file: [u16; 260],
    }

    impl Clone for PROCESSENTRY32 {
        fn clone(&self) -> Self {
            return PROCESSENTRY32{
                size: self.size,
                usage: self.usage,
                pid: self.pid,
                heap: self.heap,
                module: self.module,
                threads: self.threads,
                ppid: self.ppid,
                priority: self.priority,
                flags: self.flags,
                file: self.file,
            };
        }
    }

    #[link(name = "kernel32")]
    extern "stdcall" {
        pub fn GetLastError() -> u32;
        pub fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> HANDLE;
        pub fn CloseHandle(handle: HANDLE) -> i32;
        pub fn Process32FirstW(handle: HANDLE, entry: *mut PROCESSENTRY32) -> i32;
        pub fn Process32NextW(handle: HANDLE, entry: *mut PROCESSENTRY32) -> i32;
        pub fn OpenProcess(desired: u32, inherit: i32, pid: u32) -> HANDLE;
        pub fn QueryFullProcessImageNameW(process: HANDLE, flags: u32, name: *mut u16, size: &mut u32) -> i32;
    }
}

fn snapshot() -> Result<Vec<kernel32::PROCESSENTRY32>, String> {
    let mut processes: Vec<kernel32::PROCESSENTRY32> = Vec::new();
    unsafe {
        let snapshot = kernel32::CreateToolhelp32Snapshot(0x00000002, 0);
        if (snapshot as i32) < 0 {
            return Err(format!("kernel32::CreateToolhelp32Snapshot was failed: E{}", kernel32::GetLastError()));
        }

        let mut entry = Box::new(kernel32::PROCESSENTRY32{
            size: ::std::mem::size_of::<kernel32::PROCESSENTRY32>() as u32,
            usage: 0,
            pid: 0,
            ppid: 0,
            heap: 0,
            module: 0,
            threads: 0,
            priority: 0,
            flags: 0,
            file: [0; 260],
        });

        // Get First Process.
        let mut result = kernel32::Process32FirstW(snapshot, &mut *entry);
        if result == 0 {
            // Failure.

            // Close Handle.
            kernel32::CloseHandle(snapshot);

            // Error Code.
            let error = kernel32::GetLastError();
            if error == 0x00000012 {
                // ERROR_NO_MORE_FILES
                return Ok(processes);
            } else {
                // Return Error.
                return Err(format!("kernel32::Process32FirstW was failed: E{}", kernel32::GetLastError()));
            }
        } else {
            processes.push(*(entry.clone()));
        }

        // Read Next Processes
        loop {
            result = kernel32::Process32NextW(snapshot, &mut *entry);
            if result == 0 {
                // Failure.
                kernel32::CloseHandle(snapshot);

                // Error Code.
                let error = kernel32::GetLastError();
                if error == 0x00000012 {
                    // ERROR_NO_MORE_FILES
                    break;
                } else {
                    // Return Error.
                    return Err(format!("kernel32::Process32FirstW was failed: E{}", kernel32::GetLastError()));
                }
            }
            processes.push(*(entry.clone()));
        }

        kernel32::CloseHandle(snapshot);
    }

    return Ok(processes);
}

fn query_name(pid: u32) -> Result<String, String> {
    unsafe {
        // 0x00000400 = PROCESS_QUERY_INFORMATION
        let process = kernel32::OpenProcess(0x00000400, 0, pid);
        if (process as i32) >= 0 {
            let mut name: [u16; 260] = [0; 260];
            let mut size = 260;
            let result = kernel32::QueryFullProcessImageNameW(process, 0, &mut name[0], &mut size);

            // Close Process Handle.
            kernel32::CloseHandle(process);

            // Check Result.
            if result == 0 {
                // Failed.
                return Err(format!("QueryFullProcessImageName was failed: E{}", kernel32::GetLastError()));
            }

            // To String
            return Ok(String::from_utf16_lossy(&name).trim_right_matches(0x00 as char).to_string());
        } else {
            return Err(format!("OpenProcess was failed: E{}", kernel32::GetLastError()));
        }
    }
}

pub fn find(pid: i32) -> Result<super::ProcessEntry, String> {
    match snapshot() {
        Ok(processes) => {
            for ps in processes {
                if ps.pid == pid as u32 {
                    // Found
                    let mut entry = super::ProcessEntry{
                        pid:  ps.pid  as i32,
                        ppid: ps.ppid as i32,
                        path: String::from_utf16_lossy(&ps.file).trim_right_matches(0x00 as char).to_string(),
                    };

                    // Check Full Path.
                    if let Ok(full) = query_name(ps.pid) {
                        entry.path = full;
                    }

                    return Ok(entry);
                }
            }
            return Err("Not Found".to_string());
        },
        Err(msg) => {
            return Err(msg);
        }
    };
}
