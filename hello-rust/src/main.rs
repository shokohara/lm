use windows::{
    Win32::Foundation::*,
    core::*, Data::Xml::Dom::*, Win32::System::Threading::*,
    Graphics::Capture::*,
    Win32::UI::WindowsAndMessaging::*,
    Graphics::Capture::*,
    Win32::Graphics::Direct3D::{
        D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE,
    },
    Graphics::DirectX::DirectXPixelFormat,
    Win32::Graphics::Direct3D11::{
        D3D11CreateDevice,
        D3D11_SDK_VERSION,
        D3D11_CREATE_DEVICE_BGRA_SUPPORT, ID3D11Device, D3D11_CREATE_DEVICE_FLAG,
    },
};

use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use windows::UI::WindowId;

unsafe extern "system" fn enum_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    let mut buf = [0u16; 1024];
    if IsWindowVisible(hwnd).as_bool() && GetWindowTextW(hwnd, PWSTR(buf.as_mut_ptr()), 1024) > 0 && decode(&buf) == "League of Legends" {
        let win_text = decode(&buf);
        println!("{}", win_text);
        let item = GraphicsCaptureItem::TryCreateFromWindowId(GetWindowLongA(hwnd, GWL_ID))?;
        let d = create_d3d_device()?;
        let p:Direct3D11CaptureFramePool = Direct3D11CaptureFramePool::Create(d,DirectXPixelFormat::B8G8R8X8UIntNormalized,1,item.Size);
        // p.
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

fn decode(source: &[u16]) -> String {
    decode_utf16(source.iter().take_while(|&i| *i != 0).cloned()).map(|r| r.unwrap_or(REPLACEMENT_CHARACTER)).collect()
}

pub fn create_d3d_device() -> windows::Result<ID3D11Device> {
    let mut device = None;
    let mut result = create_d3d_device_with_type(
        D3D_DRIVER_TYPE_HARDWARE,
        D3D11_CREATE_DEVICE_BGRA_SUPPORT,
        &mut device,
    );
    unsafe {
        D3D11CreateDevice(None, D3D_DRIVER_TYPE_HARDWARE, None, D3D11_CREATE_DEVICE_BGRA_SUPPORT, std::ptr::null(), 0,D3D11_SDK_VERSION as u32, &mut device, std::ptr::null_mut(), std::ptr::null_mut())
    }
    // if let Err(error) = &result {
    //     if error.code() == DXGI_ERROR_UNSUPPORTED {
    //         result = create_d3d_device_with_type(
    //             D3D_DRIVER_TYPE_WARP,
    //             D3D11_CREATE_DEVICE_BGRA_SUPPORT,
    //             &mut device,
    //         );
    //     }
    // }
    result?;
    Ok(device.unwrap())
}

fn create_d3d_device_with_type(
    driver_type: D3D_DRIVER_TYPE,
    flags: D3D11_CREATE_DEVICE_FLAG,
    device: *mut Option<ID3D11Device>,
) -> windows::Result<()> {
    unsafe {
        D3D11CreateDevice(
            None,
            driver_type,
            None,
            flags,
            std::ptr::null(),
            0,
            D3D11_SDK_VERSION as u32,
            device,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    }
}