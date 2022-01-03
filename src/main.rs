#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use windows_sys::Win32::Foundation::{LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::System::Environment::GetCommandLineA;
use windows_sys::Win32::System::SystemInformation::GetTickCount;
use windows_sys::Win32::System::Threading::ExitProcess;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, MSG, WH_MOUSE_LL,
};

extern crate static_vcruntime;

static mut THRESHOLD: u32 = 28; // default threshold

const WM_LBUTTONDOWN: usize = 0x0201;
const WM_LBUTTONUP: usize = 0x0202;

const WM_RBUTTONDOWN: usize = 0x0204;
const WM_RBUTTONUP: usize = 0x0205;

unsafe extern "system" fn low_level_mouse_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    static mut WM_LB_LAST_DOWN: u32 = 0;
    static mut WM_LB_LAST_UP: u32 = 0;
    static mut WM_RB_LAST_DOWN: u32 = 0;
    static mut WM_RB_LAST_UP: u32 = 0;
    if code >= 0 {
        let tick = GetTickCount();
        match wparam {
            WM_LBUTTONDOWN => {
                if !(tick - WM_LB_LAST_DOWN >= THRESHOLD && tick - WM_LB_LAST_UP >= THRESHOLD) {
                    return 1;
                } else {
                    WM_LB_LAST_DOWN = tick;
                }
            }
            WM_LBUTTONUP => {
                if !(tick - WM_LB_LAST_UP >= THRESHOLD) {
                    return 1;
                } else {
                    WM_LB_LAST_UP = tick;
                }
            }
            WM_RBUTTONDOWN => {
                if !(tick - WM_RB_LAST_DOWN >= THRESHOLD && tick - WM_RB_LAST_UP >= THRESHOLD) {
                    return 1;
                } else {
                    WM_RB_LAST_DOWN = tick;
                }
            }
            WM_RBUTTONUP => {
                if !(tick - WM_RB_LAST_UP >= THRESHOLD) {
                    return 1;
                } else {
                    WM_RB_LAST_UP = tick;
                }
            }
            _ => (),
        }
    }
    CallNextHookEx(0, code, wparam, lparam)
}

#[no_mangle]
extern "C" fn mainCRTStartup() -> u32 {
    unsafe {
        const ARG_BUF_LEN: usize = 1024;
        const SPACE: u8 = 32;

        let mut args_p = GetCommandLineA();
        let mut buf = [0u8; ARG_BUF_LEN];
        let mut i = 0;
        let mut arg_start_i = 0;
        while *args_p != 0 {
            buf[i] = *args_p;
            i += 1;
            if *args_p == SPACE {
                arg_start_i = i
            }
            args_p = args_p.offset(1);
        }

        if arg_start_i != 0 {
            let arg = core::str::from_utf8_unchecked(&buf[arg_start_i..i]);
            if let Ok(t) = arg.parse() {
                THRESHOLD = t
            }
        }

        static mut DEFAULT_MSG: MSG = MSG {
            hwnd: 0,
            lParam: 0,
            message: 0,
            pt: POINT { x: 0, y: 0 },
            time: 0,
            wParam: 0,
        };
        SetWindowsHookExW(WH_MOUSE_LL, Some(low_level_mouse_proc), 0, 0);
        GetMessageW(&mut DEFAULT_MSG, 0, 0, 0);
        0
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { ExitProcess(1) }
    loop {}
}
