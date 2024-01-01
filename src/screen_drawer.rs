#![allow(dead_code)]
use raqote::*;
use std::collections::HashMap;
use std::hash::Hash;

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;
const LINE_WIDTH: f32 = 35.0;
const PCT_FACTOR: f32 = 100.0;

pub(crate) struct ScreenDrawer<'a, H>
where
    H: Hash + Eq + Clone,
{
    width: i32,
    height: i32,
    background: (u8, u8, u8, u8),
    strokes: Vec<(Box<dyn Fn(f32, f32) -> Path>, Source<'a>, StrokeStyle)>,
    images: HashMap<H, Vec<u32>>,
}

impl<'a, H> Default for ScreenDrawer<'a, H>
where
    H: Hash + Eq + Clone,
{
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            background: (0, 0, 0, 255),
            strokes: Vec::new(),
            images: HashMap::default(),
        }
    }
}

impl<'a, H> ScreenDrawer<'a, H>
where
    H: Hash + Eq + Clone,
{
    pub(crate) fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }

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

    pub(crate) fn get_image(&self, tag: H) -> Option<Vec<u32>> {
        self.images.get(&tag).cloned()
    }

    pub(crate) fn render_image(&mut self, strokes: Vec<usize>, tag: Option<H>) -> DrawTarget {
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

        if let Some(tag) = tag {
            let data = dt.get_data().to_vec();
            self.images.insert(tag, data);
        }

        dt
    }

    pub(crate) fn get_or_render_image(&mut self, strokes: Vec<usize>, tag: H) -> Vec<u32> {
        if let Some(image) = self.get_image(tag.clone()) {
            image
        } else {
            self.render_image(strokes, Some(tag.clone()));
            self.get_image(tag).unwrap()
        }
    }
}
