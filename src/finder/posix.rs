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
            return Ok(ProcessEntry{
                pid:  st.pid,
                ppid: st.ppid,
                path: st.command,
            });
        },
        Err(e) => {
            return Err(e.description().to_string());
        }
    }
}
