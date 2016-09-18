// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:

//! pentry is inspect Process Entry library inspired by mitchellh/go-ps.

mod mswin;
mod posix;

/// Process Entry Object.
pub struct ProcessEntry {
    pub pid:  i32,
    pub ppid: i32,
    pub path: String,
}

pub fn find(pid: i32) -> Result<ProcessEntry, String> {
    if cfg!(windows) {
        return mswin::find(pid);
    } else {
        return Err("Not Implemented".to_string());
    }
}

/// Methods of Process Entry.
impl ProcessEntry {
}
