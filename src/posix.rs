// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:

extern crate procinfo;

use std::error::Error;
use super::Process;
use super::Problem;

fn process(entry: &procinfo::pid::Stat) -> Process {
    let mut ps = Process{
        pid:  entry.pid,
        ppid: entry.ppid,
        name: entry.command.clone(),
        path: None,
    };

    // Resolve Path.
    let link = format!("/proc/{}/exe", entry.pid);
    if let Ok(real) = ::std::fs::canonicalize(link) {
        if let Some(path) = real.to_str() {
            ps.path = Some(path.to_string());
        }
    }

    return ps;
}

pub fn find(pid: i32) -> Result<Process, Problem> {
    match procinfo::pid::stat(pid) {
        Ok(st) => { return Ok(process(&st)); },
        Err(e) => { return Err(Problem::new(e.description())); }
    }
}
