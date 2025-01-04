use std::io::Read;

use windows::core::Result;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Globalization::{MultiByteToWideChar, CP_UTF8, MULTI_BYTE_TO_WIDE_CHAR_FLAGS};
use windows::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData,
};
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_FIXED};
use windows::Win32::System::Ole::CF_UNICODETEXT;

fn clipboard(utf8_str: &mut str) -> Result<()> {
    unsafe {
        let len = MultiByteToWideChar(
            CP_UTF8,
            MULTI_BYTE_TO_WIDE_CHAR_FLAGS(0),
            utf8_str.as_bytes(),
            None,
        );
        let buf_size = TryInto::<usize>::try_into(len)? * size_of::<u16>();
        let hmem = GlobalAlloc(GMEM_FIXED, buf_size)?;
        let p_str = GlobalLock(hmem);
        let p_str = if p_str.is_null() {
            None
        } else {
            let slice: &mut [u16] =
                std::slice::from_raw_parts_mut(p_str as _, TryInto::<usize>::try_into(len)?);
            Some(slice)
        };
        _ = MultiByteToWideChar(
            CP_UTF8,
            MULTI_BYTE_TO_WIDE_CHAR_FLAGS(0),
            utf8_str.as_bytes_mut(),
            p_str,
        );
        GlobalUnlock(hmem)?;

        OpenClipboard(None)?;
        EmptyClipboard()?;
        SetClipboardData(CF_UNICODETEXT.0.into(), HANDLE(hmem.0))?;
        CloseClipboard()?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    clipboard(&mut buf)
}
