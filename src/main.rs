#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use core::*;
use windows_sys::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::Environment::GetCommandLineA;
use windows_sys::Win32::System::SystemInformation::GetTickCount;
use windows_sys::Win32::System::Threading::ExitProcess;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, MSG, WH_MOUSE_LL, WM_LBUTTONDOWN, WM_LBUTTONUP,
    WM_RBUTTONDOWN, WM_RBUTTONUP,
};

extern crate static_vcruntime;

static mut THRESHOLD_LM: u32 = 28; // default threshold for left mouse button
static mut THRESHOLD_RM: u32 = 0;

const WM_LBUTTONDOWNU: usize = WM_LBUTTONDOWN as usize;
const WM_LBUTTONUPU: usize = WM_LBUTTONUP as usize;
const WM_RBUTTONDOWNU: usize = WM_RBUTTONDOWN as usize;
const WM_RBUTTONUPU: usize = WM_RBUTTONUP as usize;

unsafe extern "system" fn low_level_mouse_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    static mut LAST_DOWN_L: u32 = 0;
    static mut LAST_UP_L: u32 = 0;
    static mut LAST_DOWN_R: u32 = 0;
    static mut LAST_UP_R: u32 = 0;

    if code >= 0 {
        match wparam {
            WM_LBUTTONDOWNU => {
                let tick = GetTickCount();
                if !(tick - LAST_DOWN_L >= THRESHOLD_LM && tick - LAST_UP_L >= THRESHOLD_LM) {
                    return 1;
                } else {
                    LAST_DOWN_L = tick;
                }
            }
            WM_LBUTTONUPU => {
                let tick = GetTickCount();
                if !(tick - LAST_UP_L >= THRESHOLD_LM) {
                    return 1;
                } else {
                    LAST_UP_L = tick;
                }
            }
            WM_RBUTTONDOWNU => {
                let tick = GetTickCount();
                if !(tick - LAST_DOWN_R >= THRESHOLD_RM && tick - LAST_UP_R >= THRESHOLD_RM) {
                    return 1;
                } else {
                    LAST_DOWN_R = tick;
                }
            }
            WM_RBUTTONUPU => {
                let tick = GetTickCount();
                if !(tick - LAST_UP_R >= THRESHOLD_RM) {
                    return 1;
                } else {
                    LAST_UP_R = tick;
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
        let args = parse_args();
        THRESHOLD_LM = args.0;
        THRESHOLD_RM = args.1;
        SetWindowsHookExW(WH_MOUSE_LL, Some(low_level_mouse_proc), 0, 0);
        let mut msg: MSG = mem::zeroed();
        GetMessageW(&mut msg, 0, 0, 0);
        0
    }
}

// Wine's impl: https://github.com/wine-mirror/wine/blob/7ec5f555b05152dda53b149d5994152115e2c623/dlls/shell32/shell32_main.c#L58
unsafe fn parse_args() -> (u32, u32) {
    const SPACE: u8 = 32;
    const TAB: u8 = 9;
    const QUOTE: u8 = 34;
    const NULL: u8 = 0;

    let mut pcmdline = GetCommandLineA();
    if *pcmdline == QUOTE {
        pcmdline = pcmdline.offset(1);
        while *pcmdline != NULL {
            if *pcmdline == QUOTE {
                break;
            }
            pcmdline = pcmdline.offset(1);
        }
    } else {
        while *pcmdline != NULL && *pcmdline != SPACE && *pcmdline != TAB {
            pcmdline = pcmdline.offset(1);
        }
    }
    pcmdline = pcmdline.offset(1);
    while *pcmdline == SPACE || *pcmdline == TAB {
        pcmdline = pcmdline.offset(1);
    }

    let pcmdline_s = pcmdline;
    while *pcmdline != NULL {
        pcmdline = pcmdline.offset(1);
    }
    let bargs = slice::from_raw_parts_mut(pcmdline_s, pcmdline.offset_from(pcmdline_s) as usize);
    let mut args = str::from_utf8_unchecked_mut(bargs)
        .split_ascii_whitespace()
        .map(|arg_s| arg_s.parse::<u32>().unwrap_or_default());
    (
        args.next().unwrap_or_default(),
        args.next().unwrap_or_default(),
    )
}

#[panic_handler]
fn panic(_info: &panic::PanicInfo) -> ! {
    unsafe { ExitProcess(1) }
    loop {}
}
