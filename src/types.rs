#[derive(Debug, Clone)]
pub struct ScreenQuery {
    pub resolutions: Vec<Rect>,
    pub screens: usize,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub enum UnlockState {
    None,
    Started,
    KeyPressed,
    KeyActive,
    BackspaceActive,
    NothingToDelete,
}

#[derive(Debug, Clone)]
pub enum AuthState {
    Idle,
    Verify,
    Lock,
    Wrong,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Modifier {
    CapsLock,
    NumLock,
}

impl std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::CapsLock => "Caps Lock",
            Self::NumLock => "Num Lock",
        })
    }
}
