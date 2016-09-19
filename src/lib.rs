// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:

//! pentry is inspect Process Entry library inspired by mitchellh/go-ps.
//!
//! # Examples
//!
//! ```
//! extern crate libc;
//! extern crate pentry;
//!
//! let pid: i32;
//! unsafe {
//!     pid = libc::getpid() as i32;
//! }
//!
//! if let Ok(ps) = pentry::find(pid) {
//!     println!("#{} {}", ps.pid(), ps.path().unwrap());
//! }
//! ```

extern crate libc;

use std::error;
use std::fmt;

/// Basic Process Object.
#[derive(Debug)]
pub struct Process {
    pid: i32,
    ppid: i32,
    name: String,
    path: Option<String>,
}

impl Process {
    /// Get Process ID.
    pub fn pid(&self) -> i32 {
        return self.pid;
    }

    /// Get Parent Process ID.
    pub fn ppid(&self) -> i32 {
        return self.ppid;
    }

    /// Get Process Name.
    /// This value maybe program name.
    pub fn name(&self) -> &str {
        return self.name.as_str();
    }

    /// Get Full file path of program if provided.
    pub fn path(&self) -> Option<&str> {
        if let Some(path) = self.path.as_ref() {
            return Some(path.as_str());
        } else {
            return None;
        }
    }

    /// Get Parent Process Object.
    pub fn parent(&self) -> Result<Process, Problem> {
        return ::find(self.ppid);
    }
}

/// Problem occurred in find processes routine.
#[derive(Debug)]
pub struct Problem {
    message: String,
}

impl Problem {
    /// Create new Problem with message.
    fn new(msg: &str) -> Problem {
        return Problem { message: msg.to_string() };
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "{}", self.message);
    }
}

impl error::Error for Problem {
    fn description(&self) -> &str {
        return self.message.as_str();
    }
}


#[cfg(windows)]
mod mswin;

#[cfg(not(windows))]
mod posix;

/// Find Process by Requested Process ID.
#[cfg(windows)]
pub fn find(pid: i32) -> Result<Process, Problem> {
    return mswin::find(pid as u32);
}

/// Find Process by Requested Process ID.
#[cfg(not(windows))]
pub fn find(pid: i32) -> Result<Process, Problem> {
    return posix::find(pid);
}

/// Get Current Process Object.
pub fn current() -> Result<Process, Problem> {
    unsafe {
        let pid = libc::getpid() as i32;
        return ::find(pid);
    }
}
