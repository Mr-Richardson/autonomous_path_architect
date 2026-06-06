use miniquad::window::set_mouse_cursor;

pub(crate) struct MouseCursor {
    pub(crate) state: miniquad::CursorIcon,
}

impl MouseCursor {
    pub fn set(&mut self, cursor_icon: miniquad::CursorIcon) {
        if cursor_icon != self.state {
            set_mouse_cursor(cursor_icon);
            self.state = cursor_icon
        }
    }
}
