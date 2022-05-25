use windows::Graphics::Imaging::{BitmapAlphaMode, BitmapEncoder, BitmapPixelFormat};
use windows::{
    core::*,
    Foundation::TypedEventHandler,
    Graphics::Capture::*,
    Graphics::DirectX::DirectXPixelFormat,
    Win32::Foundation::*,
    Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE},
    Win32::Graphics::Direct3D11::{
        D3D11CreateDevice, ID3D11Device, ID3D11Resource, ID3D11Texture2D, D3D11_BIND_FLAG,
        D3D11_BIND_SHADER_RESOURCE, D3D11_CPU_ACCESS_READ, D3D11_CREATE_DEVICE_BGRA_SUPPORT,
        D3D11_CREATE_DEVICE_FLAG, D3D11_MAP_READ, D3D11_RESOURCE_MISC_FLAG, D3D11_SDK_VERSION,
        D3D11_TEXTURE2D_DESC, D3D11_USAGE_STAGING,
    },
    Win32::Graphics::Dxgi::IDXGIDevice,
    Win32::System::WinRT::Direct3D11::{
        CreateDirect3D11DeviceFromDXGIDevice, IDirect3DDxgiInterfaceAccess,
    },
    Win32::System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop,
    Win32::UI::WindowsAndMessaging::*,
    UI::WindowId,
};

use std::sync::mpsc::{channel, Receiver, Sender};
use windows::Storage::{CreationCollisionOption, FileAccessMode, StorageFolder};

use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::ptr::null;
use windows::Graphics::DirectX::Direct3D11::IDirect3DDevice;

unsafe extern "system" fn enum_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    let mut buf = [0u16; 1024];
    if IsWindowVisible(hwnd).as_bool()
        && GetWindowTextW(hwnd, PWSTR(buf.as_mut_ptr()), 1024) > 0
        && decode(&buf) == "League of Legends"
    {
        let worker_w: &mut Option<HWND> = unsafe { std::mem::transmute(_l_param) };
        worker_w.replace(hwnd);
        BOOL(0)
    } else {
        BOOL(1)
    }
}

fn main() -> Result<()> {
    unsafe {
        let mut hwnd: Option<HWND> = None;
        EnumWindows(Some(enum_proc), LPARAM(&mut hwnd as *mut _ as _));
        hoge(hwnd.unwrap());
    }
    Ok(())
}

unsafe fn hoge(hwnd: HWND) {
    let interop =
        windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>().unwrap();
    let holy: Result<GraphicsCaptureItem> = unsafe { interop.CreateForWindow(hwnd) };
    let item = holy.unwrap();
    let item_size = item.Size().unwrap();
    let d: ID3D11Device = create_d3d_device().unwrap();
    let dxgi_device: IDXGIDevice = d.cast().unwrap();
    let inspectable = CreateDirect3D11DeviceFromDXGIDevice(Some(dxgi_device)).unwrap();
    let dd: Result<IDirect3DDevice> = inspectable.cast();
    let frame_pool: Direct3D11CaptureFramePool = Direct3D11CaptureFramePool::CreateFreeThreaded(
        dd.unwrap(),
        DirectXPixelFormat::B8G8R8A8UIntNormalized,
        2,
        item.Size().unwrap(),
    )
    .unwrap();
    let (sender, receiver) = channel();
    let session: GraphicsCaptureSession = frame_pool.CreateCaptureSession(item).unwrap();
    let d3d_context = {
        let mut d3d_context = None;
        d.GetImmediateContext(&mut d3d_context);
        d3d_context.unwrap()
    };
    println!("{}", "Before FrameArrived");
    frame_pool.FrameArrived(
        TypedEventHandler::<Direct3D11CaptureFramePool, IInspectable>::new({
            let d = d.clone();
            let d3d_context = d3d_context.clone();
            let session = session.clone();
            let sender = sender.clone();
            println!("{}", "FrameArrived");
            move |frame_pool, _| {
                println!("{}", "moved");
                let frame_pool: &Direct3D11CaptureFramePool = frame_pool.as_ref().unwrap();
                println!("{}", "flame_pool");
                let frame: Direct3D11CaptureFrame = frame_pool.TryGetNextFrame().unwrap();
                println!("{}", "frame");
                let access: IDirect3DDxgiInterfaceAccess =
                    (&frame.Surface().unwrap()).cast().unwrap();
                println!("{}", "access");
                let object: ID3D11Texture2D =
                    unsafe { access.GetInterface::<ID3D11Texture2D>().unwrap() };
                println!("{}", "object");
                let source_texture: ID3D11Texture2D = object;
                println!("{}", "st");
                let mut desc = D3D11_TEXTURE2D_DESC::default();
                println!("{}", "before desc");
                source_texture.GetDesc(&mut desc);
                println!("{}", "desc");
                desc.BindFlags = 0;
                desc.MiscFlags = 0;
                desc.Usage = D3D11_USAGE_STAGING;
                desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
                let copy_texture = d.CreateTexture2D(&desc, std::ptr::null()).unwrap();
                println!("{}", "ct");
                d3d_context.CopyResource(
                    Some(copy_texture.cast().unwrap()),
                    Some(source_texture.cast().unwrap()),
                );
                println!("{}", "before session close");
                session.Close();
                println!("{}", "after session close");
                frame_pool.Close();
                println!("{}", "after pool close");
                sender.send(copy_texture).unwrap();
                println!("{}", "after send close");
                Ok(())
            }
        }),
    );
    session.StartCapture();
    println!("{}", "AfterStartCapture");
    let texture = receiver.recv().unwrap();
    println!("{}", "texture");
    let bits = unsafe {
        let mut desc = D3D11_TEXTURE2D_DESC::default();
        texture.GetDesc(&mut desc as *mut _);

        let resource: ID3D11Resource = texture.cast().unwrap();
        let mapped = d3d_context
            .Map(Some(resource.clone()), 0, D3D11_MAP_READ, 0)
            .unwrap();

        // Get a slice of bytes
        let slice: &[u8] = {
            std::slice::from_raw_parts(
                mapped.pData as *const _,
                (desc.Height * mapped.RowPitch) as usize,
            )
        };

        let bytes_per_pixel = 4;
        let mut bits = vec![0u8; (desc.Width * desc.Height * bytes_per_pixel) as usize];
        for row in 0..desc.Height {
            let data_begin = (row * (desc.Width * bytes_per_pixel)) as usize;
            let data_end = ((row + 1) * (desc.Width * bytes_per_pixel)) as usize;
            let slice_begin = (row * mapped.RowPitch) as usize;
            let slice_end = slice_begin + (desc.Width * bytes_per_pixel) as usize;
            bits[data_begin..data_end].copy_from_slice(&slice[slice_begin..slice_end]);
        }

        d3d_context.Unmap(Some(resource), 0);

        bits
    };
    let path = std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();
    println!("{}", path);
    let folder: StorageFolder = StorageFolder::GetFolderFromPathAsync(path.as_str())
        .unwrap()
        .get()
        .unwrap();
    let file = folder
        .CreateFileAsync("screenshot.png", CreationCollisionOption::ReplaceExisting)
        .unwrap()
        .get()
        .unwrap();
    {
        let stream = file
            .OpenAsync(FileAccessMode::ReadWrite)
            .unwrap()
            .get()
            .unwrap();
        let encoder: BitmapEncoder =
            BitmapEncoder::CreateAsync(BitmapEncoder::PngEncoderId().unwrap(), stream)
                .unwrap()
                .get()
                .unwrap();
        encoder
            .SetPixelData(
                BitmapPixelFormat::Bgra8,
                BitmapAlphaMode::Premultiplied,
                item_size.Width as u32,
                item_size.Height as u32,
                1.0,
                1.0,
                &bits,
            )
            .unwrap();

        encoder.FlushAsync().unwrap().get().unwrap();
    }
}

fn decode(source: &[u16]) -> String {
    decode_utf16(source.iter().take_while(|&i| *i != 0).cloned())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}

pub fn create_d3d_device() -> Result<ID3D11Device> {
    let mut device = None;
    let mut result = create_d3d_device_with_type(
        D3D_DRIVER_TYPE_HARDWARE,
        D3D11_CREATE_DEVICE_BGRA_SUPPORT,
        &mut device,
    );
    unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            std::ptr::null(),
            0,
            D3D11_SDK_VERSION as u32,
            &mut device,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
    }
    result.unwrap();
    Ok(device.unwrap())
}

fn create_d3d_device_with_type(
    driver_type: D3D_DRIVER_TYPE,
    flags: D3D11_CREATE_DEVICE_FLAG,
    device: *mut Option<ID3D11Device>,
) -> Result<()> {
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
