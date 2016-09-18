// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:

//! pentry is inspect Process Entry library inspired by mitchellh/go-ps.

extern crate libc;

/// Process Entry Object.
pub struct ProcessEntry {
    pub pid:  i32,
    pub ppid: i32,
    pub path: String,
}

#[cfg(windows)]
mod finder {
    mod mswin;

    pub fn find(pid: i32) -> Result<super::ProcessEntry, String> {
        match mswin::find(pid) {
            Ok(entry) => {
                return Ok(super::ProcessEntry{
                    pid:  entry.pid  as i32,
                    ppid: entry.ppid as i32,
                    path: entry.path,
                });
            },
            Err(e) => {
                return Err(e);
            }
        };
    }
}

#[cfg(not(windows))]
mod finder {
    mod posix;

    pub fn find(pid: i32) -> Result<super::ProcessEntry, String> {
        match posix::find(pid) {
            Ok(entry) => {
                return Ok(super::ProcessEntry{
                    pid: entry.pid,
                    ppid: entry.ppid,
                    path: entry.path,
                });
            },
            Err(e) => {
                return Err(e);
            }
        };
    }
}

pub use finder::*;

pub fn current() -> Result<ProcessEntry, String> {
    let pid: i32;
    unsafe {
        pid = libc::getpid() as i32;
    }
    return self::find(pid);
}
