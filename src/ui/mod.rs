use macroquad::window::screen_width;

pub mod left;
pub mod middle;
pub mod right;

pub(crate) struct Manager {
    left: left::Left,
    middle: middle::Middle,
    right: right::Right,
}

impl Manager {
    pub(crate) fn new(left: left::Left, middle: middle::Middle, right: right::Right) -> Self {
        Manager { left, middle, right }
    }

    pub(crate) fn render(&mut self) {
        self.middle.render(self.left.width, screen_width() - self.right.width);
        self.right.render();
        self.left.render();
    }

    pub(crate) fn resize_check(&mut self, tolerance: f32) {
        self.right.resize_check(tolerance);
    }
}
