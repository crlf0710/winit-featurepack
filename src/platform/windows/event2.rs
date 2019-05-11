use winit::event::OsSpecificWindowEvent;
use winit::platform::windows::OsSpecificWindowEventExtWindows;

pub trait OsSpecificWindowEvent2ExtWindows {
    fn downcast_command_event(&self) -> Option<CommandEventArgs>;
}

impl OsSpecificWindowEvent2ExtWindows for OsSpecificWindowEvent {
    fn downcast_command_event(&self) -> Option<CommandEventArgs> {
        use winapi::um::winuser::WM_COMMAND;
        if self.message() == WM_COMMAND {
            Some(CommandEventArgs(self))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, )]
pub struct CommandEventArgs<'a>(pub(crate) &'a OsSpecificWindowEvent);

impl<'a> CommandEventArgs<'a> {
    pub fn id(&self) -> u16 {
        use winapi::shared::minwindef::LOWORD;
        LOWORD(self.0.wparam() as u32)
    }

    pub fn is_menu_command(&self) -> bool {
        use winapi::shared::minwindef::HIWORD;
        HIWORD(self.0.wparam() as u32) == 0 && self.0.lparam() == 0
    }

    pub fn is_accelerator_command(&self) -> bool {
        use winapi::shared::minwindef::HIWORD;
        HIWORD(self.0.wparam() as u32) == 1 && self.0.lparam() == 0
    }

    pub fn is_control_command(&self) -> bool {
        self.0.lparam() != 0
    }
}
