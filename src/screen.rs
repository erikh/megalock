use crate::types::Rect;

pub trait Screen {
    fn width(&mut self) -> u32;
    fn height(&mut self) -> u32;
}

impl Screen for Rect {
    fn width(&mut self) -> u32 {
        self.width.into()
    }

    fn height(&mut self) -> u32 {
        self.width.into()
    }
}
