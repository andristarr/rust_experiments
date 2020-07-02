
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
use self::kernel32::GetModuleHandleW;
use winapi::HMODULE;

fn win32_string(value : &str ) -> Vec<u16> {
    OsStr::new( value ).encode_wide().chain( once( 0 ) ).collect()
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    println!("========================================");
    println!("=====  Rust Windows experiment #1  =====");
    println!("========================================");

    let module_name = win32_string("ntdll.dll");

    let h_instance: HMODULE;

    unsafe {
        h_instance = GetModuleHandleW(module_name.as_ptr());
    }

    println!("ntdll was loaded at: {:#?}", h_instance);

    pause();
}
