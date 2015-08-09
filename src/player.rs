#[derive(Copy, Clone, Debug)]
pub enum Player {
    YeonSeung,
    JunSeok,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match *self {
            Player::YeonSeung => Player::JunSeok,
            Player::JunSeok => Player::YeonSeung,
        }
    }
}

