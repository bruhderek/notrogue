use std::{collections::VecDeque, time::{Duration, Instant}};

pub enum Direction {
    K, J, H, L
}

pub enum ControllerMessage {
    Move(i32, i32),
    Teleport(i32, i32)
}

pub struct PlayerController {
    pub steps: VecDeque<Direction>,
    last_move: Option<Instant>,
    pub delay_millis: u64
}

impl PlayerController {
    pub fn new() -> Self {
        PlayerController { steps: VecDeque::new(), last_move: None, delay_millis: 200 }
    }

    pub fn on_press_key(
        &mut self,
        event: &notcurses::Input,
    ) {
        if event.is_press() || event.is_repeat() {
            if event.is_char('k') { self.push_step(Direction::K); }
            if event.is_char('j') { self.push_step(Direction::J); }
            if event.is_char('h') { self.push_step(Direction::H); }
            if event.is_char('l') { self.push_step(Direction::L); }
        }
    }

    fn push_step(&mut self, direction: Direction) {
        self.steps.push_back(direction);
    }

    pub fn on_update(&mut self) -> Vec<ControllerMessage> {
        let mut ret = Vec::new();
        while !self.steps.is_empty() {
            if let Some(last_move) = &mut self.last_move {
                if Instant::now().duration_since(*last_move).as_millis() < self.delay_millis.into() {
                    break;
                }
                *last_move += Duration::from_millis(self.delay_millis);
                match *self.steps.front().unwrap() {
                    Direction::K => ret.push(ControllerMessage::Move(0, -1)),
                    Direction::J => ret.push(ControllerMessage::Move(0, 1)),
                    Direction::H => ret.push(ControllerMessage::Move(-1, 0)),
                    Direction::L => ret.push(ControllerMessage::Move(1, 0)),
                }
                self.steps.pop_front();
            } else {
                self.last_move = Some(Instant::now());
            }
        }
        if self.steps.is_empty() {
            self.last_move = None;
        }
        ret
    }
}