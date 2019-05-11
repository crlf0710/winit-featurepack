use crate::platform_impl::windows::menu as menu_impl;
use winit::platform::windows::WindowExtWindows;

pub struct Menu {
    menu: menu_impl::Menu
}

impl Menu {
    pub fn set_command_enabled(&self, cmd: u16, enabled: bool) {
        menu_impl::set_command_enabled(self.menu.get_hmenu(), cmd, enabled).unwrap_or(());
    }

    pub fn set_command_checked(&self, cmd: u16, checked: bool) {
        menu_impl::set_command_checked(self.menu.get_hmenu(), cmd, checked).unwrap_or(());
    }
}

use std::fmt;

impl fmt::Debug for Menu {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.pad("Menu { .. }")
    }
}

pub trait MenuExtWindows {
    fn set_menu_from_resource(&self, res_id: u16) -> ();

    fn menu(&self) -> Option<Menu>;
}

impl MenuExtWindows for winit::window::Window {
    fn set_menu_from_resource(&self, res_id: u16) -> () {
        menu_impl::set_window_menu_from_resource(self.get_hwnd() as _, res_id)
            .unwrap_or(());
    }
    fn menu(&self) -> Option<Menu> {
        if let Some(menu) = menu_impl::get_window_menu(self.get_hwnd() as _).unwrap_or(None) {
            Some(Menu { menu })
        } else {
            None
        }
    }
}

