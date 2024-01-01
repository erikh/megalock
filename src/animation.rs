#![allow(dead_code)]
use crate::{screen_drawer::ScreenDrawer, wm::Event};
use raqote::{LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle};

pub(crate) trait Animation {
    fn animate(&mut self, event: crate::wm::Event) -> &[u32];
    fn last_frame(&self) -> &[u32];
}

pub(crate) type Color = (u8, u8, u8, u8);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum StarPosition {
    TopLeft,
    Top,
    TopRight,
    Center,
    Full,
}

pub(crate) struct StarAnimation<'a> {
    red: StarDrawing<'a>,
    green: StarDrawing<'a>,
    white: StarDrawing<'a>,
    last_frame: Vec<u32>,
}

const RED: Color = (200, 50, 50, 255);
const GREEN: Color = (50, 200, 50, 255);
const WHITE: Color = (255, 255, 255, 255);

impl StarAnimation<'_> {
    pub(crate) fn new(width: i32, height: i32, scale: i32, line_width: i32) -> Self {
        Self {
            red: StarDrawing::new(width, height, RED, scale, line_width),
            green: StarDrawing::new(width, height, GREEN, scale, line_width),
            white: StarDrawing::new(width, height, WHITE, scale, line_width),
            last_frame: Vec::new(),
        }
    }
}

pub(crate) struct StarDrawing<'a> {
    sd: ScreenDrawer<'a, StarPosition>,
    width: i32,
    height: i32,
    scale: i32,
    line_width: i32,
    color: Color,
}

impl StarDrawing<'_> {
    pub(crate) fn new(width: i32, height: i32, color: Color, scale: i32, line_width: i32) -> Self {
        Self {
            sd: ScreenDrawer::new(width, height),
            width,
            height,
            scale,
            color,
            line_width,
        }
    }

    fn init(&mut self) {
        let color = Source::Solid(SolidSource {
            r: self.color.0,
            g: self.color.1,
            b: self.color.2,
            a: self.color.3,
        });

        let line_style = StrokeStyle {
            cap: LineCap::Round,
            join: LineJoin::Round,
            width: self.line_width as f32,
            ..Default::default()
        };

        let pct = (self.width / self.height * self.scale) as f32;

        self.sd.add_stroke(
            move |w, h| {
                let mut pb = PathBuilder::new();
                pb.move_to(w / 2.0, h / 2.0 - pct);
                pb.line_to(w / 2.0, h / 2.0 + pct);
                pb.finish()
            },
            color.clone(),
            line_style.clone(),
        );

        self.sd.render_image(vec![0], Some(StarPosition::Top));

        self.sd.add_stroke(
            move |w, h| {
                let mut pb = PathBuilder::new();
                pb.move_to(w / 2.0 - pct, h / 2.0);
                pb.line_to(w / 2.0 + pct, h / 2.0);
                pb.finish()
            },
            color.clone(),
            line_style.clone(),
        );

        self.sd.render_image(vec![1], Some(StarPosition::Center));

        self.sd.add_stroke(
            move |w, h| {
                let mut pb = PathBuilder::new();
                pb.move_to(w / 2.0 - pct / 1.5, h / 2.0 - pct / 1.5);
                pb.line_to(w / 2.0 + pct / 1.5, h / 2.0 + pct / 1.5);
                pb.finish()
            },
            color.clone(),
            line_style.clone(),
        );

        self.sd.render_image(vec![2], Some(StarPosition::TopLeft));

        self.sd.add_stroke(
            move |w, h| {
                let mut pb = PathBuilder::new();
                pb.move_to(w / 2.0 + pct / 1.5, h / 2.0 - pct / 1.5);
                pb.line_to(w / 2.0 - pct / 1.5, h / 2.0 + pct / 1.5);
                pb.finish()
            },
            color,
            line_style,
        );

        self.sd.render_image(vec![3], Some(StarPosition::TopRight));

        self.sd
            .render_image(vec![0, 1, 2, 3], Some(StarPosition::Full));
    }

    pub(crate) fn get_image(&self, pos: StarPosition) -> Vec<u32> {
        self.sd.get_image(pos).unwrap()
    }
}

impl Animation for StarAnimation<'_> {
    fn animate(&mut self, event: Event) -> &[u32] {
        self.last_frame = match event {
            Event::KeyPressed => self.white.get_image(
                [
                    StarPosition::Top,
                    StarPosition::Center,
                    StarPosition::TopLeft,
                    StarPosition::TopRight,
                ][rand::random::<usize>() % 4]
                    .clone(),
            ),
            Event::UnlockAttempted => self.white.get_image(StarPosition::Full),
            Event::UnlockFailure => self.red.get_image(StarPosition::Full),
            Event::UnlockSuccessful => self.green.get_image(StarPosition::Full),
        };
        &self.last_frame
    }

    fn last_frame(&self) -> &[u32] {
        &self.last_frame
    }
}
