use windows::{
    Win32::Foundation::*,
    core::*, Data::Xml::Dom::*, Win32::System::Threading::*,
   Graphics::Capture::*,
    Win32::UI::WindowsAndMessaging::*,
};
// use winapi::{
//     windef::{HWND},
//     shared::{minwindef::{PLARAM,BOOL,TRUE}}
// };

use std::char::{decode_utf16,REPLACEMENT_CHARACTER};

unsafe extern "system" fn enum_proc(hwnd:HWND, _l_param:LPARAM) -> BOOL {
    let mut buf = [0u16;1024];
    if IsWindowVisible(hwnd).as_bool() && GetWindowTextW(hwnd, PWSTR(buf.as_mut_ptr()), 1024)>0 && decode(&buf) == "League of Legends"{
        let win_text = decode(&buf);
        println!("{}",win_text);
        Graphic
    }
    BOOL(1)
}

fn main() -> Result<()> {
    unsafe {
        let hWndParent = GetDesktopWindow();
        EnumWindows(Some(enum_proc), LPARAM(20));
    }
    Ok(())
}

fn decode(source:&[u16])->String{
    decode_utf16(source.iter().take_while(|&i|*i != 0).cloned()).map(|r| r.unwrap_or(REPLACEMENT_CHARACTER)).collect()
}