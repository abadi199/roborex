use direction::Direction;

#[derive(Debug)]
pub enum PlayerState {
    Standing(Direction),
    Walking {
        direction: Direction,
        grid_count: u32,
        timer: f64,
        sprites_idx: usize,
        tick: f64,
    },
}

impl PlayerState {
    pub fn sprites_idx(&mut self, new_sprites_idx: usize) {
        if let PlayerState::Walking {
            ref mut sprites_idx,
            ..
        } = self
        {
            *sprites_idx = new_sprites_idx;
        }
    }

    pub fn timer(&mut self, new_timer: f64) {
        if let PlayerState::Walking { ref mut timer, .. } = self {
            *timer = new_timer;
        }
    }

    pub fn grid_count(&mut self, new_grid_count: u32) {
        if let PlayerState::Walking {
            ref mut grid_count, ..
        } = self
        {
            *grid_count = new_grid_count;
        }
    }

    pub fn tick(&mut self, new_tick: f64) {
        if let PlayerState::Walking { ref mut tick, .. } = self {
            *tick = new_tick;
        }
    }
}

impl PlayerState {
    pub fn stop(current_state: &Self) -> Self {
        match current_state {
            PlayerState::Walking { direction, .. } => PlayerState::Standing(direction.clone()),
            PlayerState::Standing(direction) => PlayerState::Standing(direction.clone()),
        }
    }
}
