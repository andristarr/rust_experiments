
#![feature(const_fn)]
#![feature(libc)]

extern crate winapi;
extern crate user32;
extern crate kernel32;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::mem;
use std::ptr::null_mut;
use std::io::Error;
use std::io::{stdin, stdout, Read, Write};

use self::winapi::HWND;
use self::user32::MessageBoxW;
use self::winapi::HMODULE;
use self::winapi::MB_OK;

fn win32_string(value : &str ) -> Vec<u16> {
    OsStr::new( value ).encode_wide().chain( once( 0 ) ).collect()
}

// entry point
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn DllMain(
    dll_module: winapi::HINSTANCE,
    call_reason: winapi::DWORD,
    reserved: winapi::LPVOID)
    -> winapi::BOOL
{
    const DLL_PROCESS_ATTACH: winapi::DWORD = 1;
    const DLL_PROCESS_DETACH: winapi::DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => init(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    return winapi::TRUE;
}

fn init() {
    unsafe {
        MessageBoxW(null_mut(), win32_string("Injected successfully").as_ptr(),
                    win32_string("Injection message").as_ptr(), MB_OK);
    }
}