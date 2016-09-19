// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:
#![allow(non_camel_case_types)]

extern crate libc;

pub type HANDLE  = *mut libc::c_void;
pub type HMODULE = HANDLE;

pub const MAX_PATH: usize = 260;

pub const ERROR_NO_MORE_FILES: u32 = 0x00000012;
pub const ERROR_NOT_FOUND: u32 = 0x00000490;

pub const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;

pub const TH32CS_SNAPPROCESS: u32 = 0x00000002;

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

impl PROCESSENTRY32 {
    pub fn new() -> PROCESSENTRY32 {
        return PROCESSENTRY32{
            size: ::std::mem::size_of::<PROCESSENTRY32>() as u32,
            usage: 0,
            pid: 0,
            heap: 0,
            module: 0,
            threads: 0,
            ppid: 0,
            priority: 0,
            flags: 0,
            file: [0; MAX_PATH],
        };
    }
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
    pub fn FormatMessageW(flags: u32, src: *mut libc::c_void, message: u32, language: u32, buffer: *mut u16, size: u32, args: *mut libc::c_void) -> u32;
    pub fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> HANDLE;
    pub fn CloseHandle(handle: HANDLE) -> i32;
    pub fn Process32FirstW(handle: HANDLE, entry: *mut PROCESSENTRY32) -> i32;
    pub fn Process32NextW(handle: HANDLE, entry: *mut PROCESSENTRY32) -> i32;
    pub fn OpenProcess(desired: u32, inherit: i32, pid: u32) -> HANDLE;
    pub fn QueryFullProcessImageNameW(process: HANDLE, flags: u32, name: *mut u16, size: &mut u32) -> i32;
}
