use crate::{screen_drawer::ScreenDrawer, wm::Event};
use raqote::{LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle};
use tracing::trace;

#[derive(Debug, Clone)]
pub struct AnimationDriver {
    typ: AnimationType,
    last_frame: Vec<u32>,
}

impl AnimationDriver {
    pub fn new(typ: AnimationType) -> Self {
        Self {
            typ,
            last_frame: vec![0],
        }
    }

    pub fn animate(&mut self, event: Event) {
        let mut animator: Box<dyn Animation> = self.typ.clone().into();
        animator.animate(event);

        self.last_frame = self.last_frame().to_vec();
    }

    pub fn last_frame(&self) -> &[u32] {
        self.last_frame.as_slice()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnimationType {
    StarAnimation {
        width: i32,
        height: i32,
        scale: i32,
        line_width: i32,
    },
}

impl From<&AnimationType> for Box<dyn Animation> {
    fn from(value: &AnimationType) -> Self {
        value.clone().into()
    }
}

impl From<AnimationType> for Box<dyn Animation> {
    fn from(value: AnimationType) -> Self {
        match value {
            AnimationType::StarAnimation {
                width,
                height,
                scale,
                line_width,
            } => Box::new(StarAnimation::new(width, height, scale, line_width)),
        }
    }
}

pub trait Animation {
    fn animate(&mut self, event: Event) -> Vec<u32>;
}

pub(crate) type Color = (u8, u8, u8, u8);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum StarPosition {
    TopLeft,
    Top,
    TopRight,
    Center,
    X,
    Plus,
    Full,
}

pub(crate) struct StarAnimation {
    width: i32,
    height: i32,
    scale: i32,
    line_width: i32,
}

const RED: Color = (200, 50, 50, 255);
const GREEN: Color = (50, 200, 50, 255);
const WHITE: Color = (255, 255, 255, 255);

impl StarAnimation {
    pub(crate) fn new(width: i32, height: i32, scale: i32, line_width: i32) -> Self {
        Self {
            width,
            height,
            scale,
            line_width,
        }
    }

    fn white(&self, position: StarPosition) -> Vec<u32> {
        star_drawing(
            self.width,
            self.height,
            WHITE,
            self.scale,
            self.line_width,
            position,
        )
    }

    fn red(&self) -> Vec<u32> {
        star_drawing(
            self.width,
            self.height,
            RED,
            self.scale,
            self.line_width,
            StarPosition::Full,
        )
    }

    fn green(&self) -> Vec<u32> {
        star_drawing(
            self.width,
            self.height,
            GREEN,
            self.scale,
            self.line_width,
            StarPosition::Full,
        )
    }
}

pub(crate) fn star_drawing(
    width: i32,
    height: i32,
    color: Color,
    scale: i32,
    line_width: i32,
    position: StarPosition,
) -> Vec<u32> {
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

    star_draw_position(sd, position)
}

fn star_draw_position(mut sd: ScreenDrawer, position: StarPosition) -> Vec<u32> {
    match position {
        StarPosition::Top => sd.render_image(vec![0]),
        StarPosition::Center => sd.render_image(vec![1]),
        StarPosition::TopLeft => sd.render_image(vec![2]),
        StarPosition::TopRight => sd.render_image(vec![3]),
        StarPosition::X => sd.render_image(vec![2, 3]),
        StarPosition::Plus => sd.render_image(vec![0, 1]),
        StarPosition::Full => sd.render_image(vec![0, 1, 2, 3]),
    }
}

impl Animation for StarAnimation {
    fn animate(&mut self, event: Event) -> Vec<u32> {
        trace!("Animating event {:?}", event);
        let res = match event {
            Event::Reset => vec![0],
            Event::KeyPressed => self
                .white(
                    [
                        StarPosition::Top,
                        StarPosition::Center,
                        StarPosition::TopLeft,
                        StarPosition::TopRight,
                        StarPosition::Plus,
                        StarPosition::X,
                    ][rand::random::<usize>() % 6]
                        .clone(),
                )
                .clone(),
            Event::UnlockAttempted => self.white(StarPosition::Full).clone(),
            Event::UnlockFailure => self.red().clone(),
            Event::UnlockSuccessful => self.green().clone(),
        };
        trace!("Result computed, len: {}", res.len());
        res
    }
}
