use winapi::shared::windef::HWND;
use winapi::shared::windef::HMENU;
use crate::platform_impl::windows::utils::*;
use std::convert::TryInto;

pub(crate) struct Menu(HMENU);

impl Menu {
    pub(crate) fn get_hmenu(&self) -> HMENU {
        self.0
    }
}

impl std::convert::TryFrom<HMENU> for Menu {
    type Error = ();

    fn try_from(h: HMENU) -> Result<Menu, ()> {
        if !h.is_null() {
            Ok(Menu(h))
        } else {
            Err(())
        }
    }
}

pub(crate) fn set_window_menu_from_resource(hwnd: HWND, res_id: u16) -> Result<(), WinapiError> {
    use winapi::um::winuser::LoadMenuW;
    use winapi::um::winuser::SetMenu;
    unsafe {
        let h = LoadMenuW(exe_instance(), res_id as usize as _).ok_or_last_error()?;
        assert!(!h.is_null());
        SetMenu(hwnd, h).booleanize().ok_or_last_error()?;
    };
    Ok(())
}

pub(crate) fn get_window_menu(hwnd: HWND) -> Result<Option<Menu>, WinapiError> {
    use winapi::um::winuser::GetMenu;
    
    let menu = unsafe {
        let h = GetMenu(hwnd).ok_or_last_error()?;
        h.try_into().map(Some).unwrap_or(None)
    };
    Ok(menu)
}

pub(crate) fn set_command_enabled(hmenu: HMENU, cmd: u16, enabled: bool) -> Result<(), WinapiError> {
    use winapi::um::winuser::EnableMenuItem;
    use winapi::um::winuser::MF_BYCOMMAND;
    use winapi::um::winuser::MF_BYPOSITION;
    use winapi::um::winuser::MF_ENABLED;
    use winapi::um::winuser::MF_DISABLED;
    use winapi::um::winuser::MF_GRAYED;
    use crate::platform_impl::windows::utils::last_error;

    let by_command = true;
    let id_or_pos = cmd;
    let gray_or_not = true;
    unsafe {
        let h = hmenu;
        let mut f = 0;
        if by_command {
            f |= MF_BYCOMMAND;
        } else {
            f |= MF_BYPOSITION;
        }
        if enabled {
            f |= MF_ENABLED;
        } else {
            f |= if gray_or_not { MF_GRAYED } else { MF_DISABLED };
        }
        let r = EnableMenuItem(h, id_or_pos as _, f);
        if r == -1i32 as _ {
            return last_error();
        }
    }
    Ok(())
}

pub(crate) fn set_command_checked(hmenu: HMENU, cmd: u16, checked: bool) -> Result<(), WinapiError> {
    use winapi::um::winuser::CheckMenuItem;
    use winapi::um::winuser::MF_BYCOMMAND;
    use winapi::um::winuser::MF_BYPOSITION;
    use winapi::um::winuser::MF_CHECKED;
    use winapi::um::winuser::MF_UNCHECKED;
    use crate::platform_impl::windows::utils::last_error;

    let by_command = true;
    let id_or_pos = cmd;
    unsafe {
        let h = hmenu;
        let mut f = 0;
        if by_command {
            f |= MF_BYCOMMAND;
        } else {
            f |= MF_BYPOSITION;
        }
        if checked {
            f |= MF_CHECKED;
        } else {
            f |= MF_UNCHECKED;
        }
        let r = CheckMenuItem(h, id_or_pos as _, f);
        if r == -1i32 as _ {
            return last_error();
        }
    }
    Ok(())
}
