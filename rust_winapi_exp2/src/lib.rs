
#![feature(const_fn)]
#![feature(libc)]

extern crate winapi;
extern crate user32;
extern crate kernel32;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::{mem, ptr};
use std::ptr::null_mut;
use std::io::Error;
use std::io::{stdin, stdout, Read, Write};

use self::winapi::HWND;
use self::user32::MessageBoxW;
use self::winapi::HMODULE;
use self::winapi::MB_OK;
use self::winapi::DWORD;
use self::kernel32::GetCurrentProcessId;
use self::winapi::HANDLE;
use self::kernel32::OpenProcess;
use self::winapi::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use self::winapi::FALSE;
use self::kernel32::K32EnumProcessModules;
use std::borrow::BorrowMut;
use self::winapi::MAX_PATH;
use self::kernel32::K32GetModuleBaseNameW;
use self::winapi::minwindef;

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
        DLL_PROCESS_ATTACH => enum_processes(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    return winapi::TRUE;
}

fn enum_processes() {
    let pid: DWORD;
    let handle: HANDLE;
    let mut cbNeeded: DWORD = 0;

    unsafe {
        let sizeof_hmodule = std::mem::size_of::<HMODULE>();

        pid = GetCurrentProcessId();
        handle = OpenProcess(PROCESS_QUERY_INFORMATION |
            PROCESS_VM_READ, FALSE, pid);

        let mut modules = {
            K32EnumProcessModules(handle, ptr::null_mut(), 0, &mut cbNeeded);
            let num_entries_needed = cbNeeded as usize / sizeof_hmodule;
            let mut modules = Vec::<HMODULE>::with_capacity(num_entries_needed);
            modules.resize(num_entries_needed, ptr::null_mut());
            modules
        };

        {
            let mut bytes_fetched: DWORD = 0;
            let ret = K32EnumProcessModules(
                handle,
                modules.as_mut_ptr(),
                (modules.len() * sizeof_hmodule) as u32,
                &mut bytes_fetched,
            );

            let num_entries_fetched = bytes_fetched as usize / sizeof_hmodule;
            modules.resize(num_entries_fetched, ptr::null_mut());
        }

        for module in modules {
            {
                const BUF_SIZE: usize = 1024;
                let mut buf = [0u16; BUF_SIZE];

                let n = K32GetModuleBaseNameW(handle, module, buf.as_mut_ptr(), BUF_SIZE as u32);

                let mut str = String::from_utf16_lossy(&buf);
                str.truncate(n as usize);
                MessageBoxW(null_mut(), win32_string(str.as_ref()).as_ptr(),
                            win32_string("Modules").as_ptr(), MB_OK);
            }
        }
    }


}