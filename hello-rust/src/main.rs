use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml("<html>hello world</html>")?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), true, false, None);
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;

        MessageBoxA(None, "Text", "Caption", MB_OK);
    }

    Ok(())
}