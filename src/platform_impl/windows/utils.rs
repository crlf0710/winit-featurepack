use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::HINSTANCE;

pub(crate) struct WinapiError(u32);

pub(crate) fn exe_instance() -> HINSTANCE {
    use std::ptr::null_mut;
    unsafe { winapi::um::libloaderapi::GetModuleHandleW(null_mut()) }
}

pub(crate) trait Booleanize {
    fn booleanize(self) -> bool;
}

impl Booleanize for BOOL {
    fn booleanize(self) -> bool {
        self != winapi::shared::minwindef::FALSE
    }
}

pub(crate) fn last_error<T>() -> Result<T, WinapiError> {
    use winapi::um::errhandlingapi::GetLastError;
    Err(unsafe {
        WinapiError(GetLastError())
    })
}

pub(crate) trait OkOrLastError<T = Self>: Sized {
    fn ok_or_last_error(self) -> Result<T, WinapiError>;
}

impl<T> OkOrLastError for * mut T {
    fn ok_or_last_error(self) -> Result<Self, WinapiError> {
        if !self.is_null() {
            Ok(self)
        } else {
            last_error()
        }
    }
}

impl OkOrLastError<()> for bool {
    fn ok_or_last_error(self) -> Result<(), WinapiError> {
        if self {
            Ok(())
        } else {
            last_error()
        }
    }
}
