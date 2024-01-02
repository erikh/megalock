use raqote::*;

pub(crate) struct ScreenDrawer<'a> {
    width: i32,
    height: i32,
    background: (u8, u8, u8, u8),
    strokes: Vec<(Box<dyn Fn(f32, f32) -> Path>, Source<'a>, StrokeStyle)>,
}

impl<'a> Default for ScreenDrawer<'a> {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            background: (0, 0, 0, 255),
            strokes: Vec::new(),
        }
    }
}

impl<'a> ScreenDrawer<'a> {
    pub(crate) fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }

    #[allow(dead_code)] // NOTE: not currently used with black bg's (the default), but should matter later
    pub(crate) fn background(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.background = (r, g, b, a)
    }

    pub(crate) fn add_stroke(
        &mut self,
        path: impl Fn(f32, f32) -> Path + 'static,
        source: Source<'a>,
        style: StrokeStyle,
    ) -> usize {
        self.strokes.push((Box::new(path), source, style));
        self.strokes.len() - 1
    }

    pub(crate) fn render_image(&mut self, strokes: Vec<usize>) -> Vec<u32> {
        let mut dt = DrawTarget::new(self.width, self.height);
        dt.fill_rect(
            0.0,
            0.0,
            self.width as f32,
            self.height as f32,
            &Source::Solid(SolidSource {
                r: self.background.0,
                g: self.background.1,
                b: self.background.2,
                a: self.background.3,
            }),
            &DrawOptions::new(),
        );
        for stroke in &strokes {
            let stroke = &self.strokes[*stroke];
            dt.stroke(
                &stroke.0(self.width as f32, self.height as f32),
                &stroke.1,
                &stroke.2,
                &DrawOptions::new(),
            );
        }

        dt.get_data().to_vec()
    }
}
