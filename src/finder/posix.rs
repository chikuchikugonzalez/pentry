// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:

extern crate procinfo;

use std::error::Error;

pub struct ProcessEntry {
    pub pid:  i32,
    pub ppid: i32,
    pub path: String,
}

pub fn find(pid: i32) -> Result<ProcessEntry, String> {
    match procinfo::pid::stat(pid) {
        Ok(st) => {
            let mut entry = ProcessEntry{
                pid:  st.pid,
                ppid: st.ppid,
                path: st.command,
            };

            // Resolve Path.
            let exelink = format!("/proc/{}/exe", st.pid);
            if let Ok(real) = ::std::fs::canonicalize(exelink) {
                if let Some(path) = real.to_str() {
                    entry.path = path.to_string();
                }
            }

            return Ok(entry);
        },
        Err(e) => {
            return Err(e.description().to_string());
        }
    }
}
