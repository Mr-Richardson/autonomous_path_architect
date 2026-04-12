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
        Manager {
            left,
            middle,
            right,
        }
    }

    pub(crate) fn render(&mut self) {
        self.left.render();
        self.middle.render();
        self.right.render();
    }
}