// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:

//! pentry is inspect Process Entry library inspired by mitchellh/go-ps.

extern crate libc;

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Process {
    pid: i32,
    ppid: i32,
    name: String,
    path: Option<String>,
}

impl Process {
    pub fn pid(&self) -> i32 {
        return self.pid;
    }
    pub fn ppid(&self) -> i32 {
        return self.ppid;
    }
    pub fn name(&self) -> &str {
        return self.name.as_str();
    }
    pub fn path(&self) -> Option<&str> {
        if let Some(path) = self.path.as_ref() {
            return Some(path.as_str());
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
pub struct Problem {
    message: String,
}

impl Problem {
    fn new(msg: &str) -> Problem {
        return Problem { message: msg.to_string() };
    }
}

impl Display for Problem {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        return write!(formatter, "{}", self.message);
    }
}

impl Error for Problem {
    fn description(&self) -> &str {
        return self.message.as_str();
    }
}


#[cfg(windows)]
mod mswin;

#[cfg(not(windows))]
mod posix;

#[cfg(windows)]
pub fn find(pid: i32) -> Result<Process, Problem> {
    return mswin::find(pid as u32);
}

#[cfg(not(windows))]
pub fn find(pid: i32) -> Result<Process, Problem> {
    return posix::find(pid);
}
