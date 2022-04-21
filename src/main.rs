// no need to allocate a console for a long-running program that does not output anything
#![windows_subsystem = "windows"]
#![no_std]
#![no_main]

use core::*;
use windows_sys::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::Environment::GetCommandLineA;
use windows_sys::Win32::System::SystemInformation::GetTickCount;
use windows_sys::Win32::System::Threading::ExitProcess;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, WH_MOUSE_LL, WM_LBUTTONDOWN, WM_LBUTTONUP,
    WM_RBUTTONDOWN, WM_RBUTTONUP,
};

static mut THRESHOLD_LM: u32 = 30;
static mut THRESHOLD_RM: u32 = 0;

const WM_LBUTTONDOWNU: usize = WM_LBUTTONDOWN as _;
const WM_LBUTTONUPU: usize = WM_LBUTTONUP as _;
const WM_RBUTTONDOWNU: usize = WM_RBUTTONDOWN as _;
const WM_RBUTTONUPU: usize = WM_RBUTTONUP as _;

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
fn _start() {
    unsafe {
        let args = parse_args();
        if let Some(arg_lm) = args.0 {
            THRESHOLD_LM = arg_lm;
        }
        if let Some(arg_rm) = args.1 {
            THRESHOLD_RM = arg_rm;
        }
        SetWindowsHookExW(WH_MOUSE_LL, Some(low_level_mouse_proc), 0, 0);
        GetMessageW(&mut mem::zeroed(), 0, 0, 0);
    }
}

// Wine's impl: https://github.com/wine-mirror/wine/blob/7ec5f555b05152dda53b149d5994152115e2c623/dlls/shell32/shell32_main.c#L58
unsafe fn parse_args() -> (Option<u32>, Option<u32>) {
    const SPACE: u8 = b' ';
    const TAB: u8 = b'\t';
    const QUOTE: u8 = b'"';
    const NULL: u8 = b'\0';

    let mut pcmdline = GetCommandLineA();
    if *pcmdline == QUOTE {
        pcmdline = pcmdline.add(1);
        while *pcmdline != NULL {
            if *pcmdline == QUOTE {
                break;
            }
            pcmdline = pcmdline.add(1);
        }
    } else {
        while *pcmdline != NULL && *pcmdline != SPACE && *pcmdline != TAB {
            pcmdline = pcmdline.add(1);
        }
    }
    pcmdline = pcmdline.add(1);
    while *pcmdline == SPACE || *pcmdline == TAB {
        pcmdline = pcmdline.add(1);
    }
    let pcmdline_s = pcmdline;
    while *pcmdline != NULL {
        pcmdline = pcmdline.add(1);
    }

    let mut args = slice::from_raw_parts_mut(pcmdline_s, pcmdline.offset_from(pcmdline_s) as usize)
        .split(|p| p == &SPACE)
        .take(2)
        .filter_map(|v| str::from_utf8_unchecked(v).parse::<u32>().ok());

    (args.next(), args.next())
}

#[panic_handler]
fn panic(_info: &panic::PanicInfo) -> ! {
    unsafe { ExitProcess(1) }
}
