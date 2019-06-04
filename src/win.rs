use self::super::Size;

/// Gets the current terminal size
pub fn get() -> Option<Size> {
    use winapi::um::{
        handleapi::INVALID_HANDLE_VALUE, processenv::GetStdHandle,
        winbase::STD_OUTPUT_HANDLE, wincon::GetConsoleScreenBufferInfo,
    };

    let handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
    if handle == INVALID_HANDLE_VALUE {
        return None;
    }

    // https://msdn.microsoft.com/en-us/library/windows/desktop/ms683171(v=vs.85).aspx
    unsafe {
        let mut info = std::mem::zeroed();
        if GetConsoleScreenBufferInfo(handle, &mut info) != 0 {
            Some(Size {
                rows: (info.srWindow.Bottom - info.srWindow.Top + 1) as u16,
                cols: (info.srWindow.Right - info.srWindow.Left + 1) as u16,
            })
        } else {
            None
        }
    }
}
