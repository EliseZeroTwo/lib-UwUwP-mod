fn main() {
    windows::build!(
        windows::win32::system_services::{GetProcAddress, GetModuleHandleW},
        windows::win32::windows_and_messaging::{HWND, MessageBoxW},
    );
}
