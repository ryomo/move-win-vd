// Prevent a console window from appearing when launched
#![windows_subsystem = "windows"]

use std::env;
use std::process;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
use winvd::{get_current_desktop, get_desktop_count, move_window_to_desktop, switch_desktop};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let flags: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    // First positional argument must be "r" (next) or "l" (prev)
    let direction = match args.first().map(|s| s.as_str()) {
        Some("r") => 1i32,
        Some("l") => -1i32,
        _ => process::exit(1),
    };

    // /w: wrap around at edges
    let wrap = flags.contains(&"/w");

    // /s: also switch the current desktop to follow the window
    let follow = flags.contains(&"/s");

    // Get the currently focused window handle
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0.is_null() {
        process::exit(1);
    }

    // Get the current desktop index and total desktop count
    let Ok(current) = get_current_desktop() else {
        process::exit(1);
    };
    let Ok(current_index) = current.get_index() else {
        process::exit(1);
    };
    let Ok(count) = get_desktop_count() else {
        process::exit(1);
    };

    // Calculate target desktop index, optionally wrapping around
    let raw = current_index as i32 + direction;
    let target_index = if wrap {
        raw.rem_euclid(count as i32) as u32
    } else {
        if raw < 0 || raw >= count as i32 {
            process::exit(0);
        }
        raw as u32
    };

    // Move the foreground window to the target desktop
    if move_window_to_desktop(target_index, &hwnd).is_err() {
        process::exit(1);
    }

    // Optionally switch the current view to the target desktop
    if follow {
        let _ = switch_desktop(target_index);
    }
}
