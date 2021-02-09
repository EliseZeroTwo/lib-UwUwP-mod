use std::{ffi::CString, string::FromUtf16Error};

use crate::bindings::windows::win32::system_services;

pub fn pcwstr_to_string(pwstr: &mut *const u16) -> Result<String, FromUtf16Error> {
    let mut out_vec = Vec::<u16>::new();
    unsafe {
        while !pwstr.is_null() && **pwstr != 0 {
            out_vec.push(**pwstr);
            *pwstr = pwstr.add(1);
        }
    }
    String::from_utf16(&out_vec)
}

pub fn string_to_wchar_vec(val: &String) -> Vec<u16> {
    let mut out_vec = val
        .chars()
        .map(|x| {
            if x as u32 > 0xD7FF {
                panic!("Invalid pkg name");
            }

            x as u16
        })
        .collect::<Vec<u16>>();
    out_vec.push(0);
    out_vec
}

// Adapted from https://github.com/darfink/detour-rs/blob/master/examples/messageboxw_detour.rs#L47
pub fn get_module_symbol_address(module: &str, symbol: &str) -> Option<usize> {
    let module = string_to_wchar_vec(&module.to_string());
    let symbol = CString::new(symbol).unwrap();
    unsafe {
        let handle = system_services::GetModuleHandleW(module.as_ptr());
        match system_services::GetProcAddress(handle, symbol.as_ptr()) {
            Some(n) => Some(std::mem::transmute(n)),
            None => None,
        }
    }
}
