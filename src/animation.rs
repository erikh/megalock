use crate::{screen_drawer::ScreenDrawer, wm::Event};
use raqote::{LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum AnimationTypes {
    StarAnimation {
        width: i32,
        height: i32,
        scale: i32,
        line_width: i32,
    },
}

impl From<&AnimationTypes> for Box<dyn Animation> {
    fn from(value: &AnimationTypes) -> Self {
        value.clone().into()
    }
}

impl From<AnimationTypes> for Box<dyn Animation> {
    fn from(value: AnimationTypes) -> Self {
        match value {
            AnimationTypes::StarAnimation {
                width,
                height,
                scale,
                line_width,
            } => Box::new(StarAnimation::new(width, height, scale, line_width)),
        }
    }
}

pub trait Animation {
    fn animate(&mut self, event: Event) -> &[u32];
    fn last_frame(&self) -> &[u32];
}

pub(crate) type Color = (u8, u8, u8, u8);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum StarPosition {
    Blank,
    TopLeft,
    Top,
    TopRight,
    Center,
    X,
    Plus,
    Full,
}

pub(crate) struct StarAnimation {
    red: StarDrawing,
    green: StarDrawing,
    white: StarDrawing,
    last_frame: Vec<u32>,
}

const RED: Color = (200, 50, 50, 255);
const GREEN: Color = (50, 200, 50, 255);
const WHITE: Color = (255, 255, 255, 255);

impl StarAnimation {
    pub(crate) fn new(width: i32, height: i32, scale: i32, line_width: i32) -> Self {
        // FIXME: this is really wasteful for a lot of the stuff that needs to be rendered.
        Self {
            red: StarDrawing::new(width, height, RED, scale, line_width),
            green: StarDrawing::new(width, height, GREEN, scale, line_width),
            white: StarDrawing::new(width, height, WHITE, scale, line_width),
            last_frame: Vec::new(),
        }
    }
}

pub(crate) struct StarDrawing(HashMap<StarPosition, Vec<u32>>);

impl StarDrawing {
    pub(crate) fn new(width: i32, height: i32, color: Color, scale: i32, line_width: i32) -> Self {
        let mut map = HashMap::default();

        let mut sd = ScreenDrawer::new(width, height);
        let color = Source::Solid(SolidSource {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        });

        let line_style = StrokeStyle {
            cap: LineCap::Round,
            join: LineJoin::Round,
            width: line_width as f32,
            ..Default::default()
        };

        let pct = (width / height * scale) as f32;

        sd.add_stroke(
            move |w, h| {
                let mut pb = PathBuilder::new();
                pb.move_to(w / 2.0, h / 2.0 - pct);
                pb.line_to(w / 2.0, h / 2.0 + pct);
                pb.finish()
            },
            color.clone(),
            line_style.clone(),
        );

        map.insert(StarPosition::Top, sd.render_image(vec![0]));

        sd.add_stroke(
            move |w, h| {
                let mut pb = PathBuilder::new();
                pb.move_to(w / 2.0 - pct, h / 2.0);
                pb.line_to(w / 2.0 + pct, h / 2.0);
                pb.finish()
            },
            color.clone(),
            line_style.clone(),
        );

        map.insert(StarPosition::Center, sd.render_image(vec![1]));

        sd.add_stroke(
            move |w, h| {
                let mut pb = PathBuilder::new();
                pb.move_to(w / 2.0 - pct / 1.5, h / 2.0 - pct / 1.5);
                pb.line_to(w / 2.0 + pct / 1.5, h / 2.0 + pct / 1.5);
                pb.finish()
            },
            color.clone(),
            line_style.clone(),
        );

        map.insert(StarPosition::TopLeft, sd.render_image(vec![2]));

        sd.add_stroke(
            move |w, h| {
                let mut pb = PathBuilder::new();
                pb.move_to(w / 2.0 + pct / 1.5, h / 2.0 - pct / 1.5);
                pb.line_to(w / 2.0 - pct / 1.5, h / 2.0 + pct / 1.5);
                pb.finish()
            },
            color,
            line_style,
        );

        map.insert(StarPosition::TopRight, sd.render_image(vec![3]));
        map.insert(StarPosition::X, sd.render_image(vec![2, 3]));
        map.insert(StarPosition::Plus, sd.render_image(vec![0, 1]));
        map.insert(StarPosition::Full, sd.render_image(vec![0, 1, 2, 3]));
        map.insert(StarPosition::Blank, sd.render_image(vec![]));

        Self(map)
    }
}

impl Animation for StarAnimation {
    fn animate(&mut self, event: Event) -> &[u32] {
        self.last_frame = match event {
            Event::Idle => self.last_frame.clone(), // FIXME: silly
            Event::Reset => self.white.0.get(&StarPosition::Blank).unwrap().clone(),
            Event::KeyPressed => self
                .white
                .0
                .get(
                    &[
                        StarPosition::Top,
                        StarPosition::Center,
                        StarPosition::TopLeft,
                        StarPosition::TopRight,
                        StarPosition::Plus,
                        StarPosition::X,
                    ][rand::random::<usize>() % 6],
                )
                .unwrap()
                .clone(),
            Event::UnlockAttempted => self.white.0.get(&StarPosition::Full).unwrap().clone(),
            Event::UnlockFailure => self.red.0.get(&StarPosition::Full).unwrap().clone(),
            Event::UnlockSuccessful => self.green.0.get(&StarPosition::Full).unwrap().clone(),
        };
        &self.last_frame
    }

    fn last_frame(&self) -> &[u32] {
        &self.last_frame
    }
}
