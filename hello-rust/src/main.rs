//use ferris_says::say;
//use std::io::{stdout, BufWriter};
//
//fn main() {
//    let stdout = stdout();
//    let message = String::from("Hello fellow Rustaceans!");
//    let width = message.chars().count();
//
//    let mut writer = BufWriter::new(stdout.lock());
//    say(message.as_bytes(), width, &mut writer).unwrap();
//}

// use bindings::{
//     Windows::Foundation::Uri,
//     Windows::Web::Syndication::SyndicationClient,
//     Windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow,
// };
//
// fn main() -> windows::Result<()> {
//     let uri = Uri::CreateUri("https://blogs.windows.com/feed")?;
//     let client = SyndicationClient::new()?;
//     let feed = client.RetrieveFeedAsync(uri)?.get()?;
//
//     for item in feed.Items()? {
//         println!("{}", item.Title()?.Text()?);
//     }
//     let hWndParent = GetDesktopWindow::new();
//     Ok(())
// }

use windows::{
    Win32::UI::WindowsAndMessaging::*,
};
fn main() -> windows::Result<()> {
    // let uri = Uri::CreateUri("https://blogs.windows.com/feed")?;
    // let client = SyndicationClient::new()?;
    // let feed = client.RetrieveFeedAsync(uri)?.get()?;
    //
    // for item in feed.Items()? {
    //     println!("{}", item.Title()?.Text()?);
    // }
    // let hWndParent = GetDesktopWindow::new();
    Ok(())
}