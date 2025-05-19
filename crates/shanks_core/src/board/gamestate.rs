use super::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameState {
    OnGoing,
    Draw,
    Win(Color),
}

impl GameState {
    pub fn is_over(&self) -> bool {
        matches!(self, GameState::Draw | GameState::Win(_))
    }

    pub fn is_draw(&self) -> bool {
        matches!(self, GameState::Draw)
    }

    pub fn is_win(&self) -> bool {
        matches!(self, GameState::Win(_))
    }

    pub fn winner(&self) -> Option<Color> {
        match self {
            GameState::Win(color) => Some(*color),
            _ => None,
        }
    }

    pub fn is_ongoing(&self) -> bool {
        matches!(self, GameState::OnGoing)
    }
}
