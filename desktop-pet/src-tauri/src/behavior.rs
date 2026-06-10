use serde::{Serialize, Deserialize};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BehaviorAction {
    Idle,
    WalkLeft,
    WalkRight,
    Jump,
    Sleep,
    Sit,
    Climb,
    React(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BehaviorState {
    pub current_action: BehaviorAction,
    pub action_timer: f64,
    pub action_duration: f64,
    pub energy: f64,
    pub happiness: f64,
    pub boredom: f64,
    pub last_interaction: f64,
}

impl BehaviorState {
    pub fn new() -> Self {
        BehaviorState {
            current_action: BehaviorAction::Idle,
            action_timer: 0.0,
            action_duration: 3.0,
            energy: 100.0,
            happiness: 80.0,
            boredom: 0.0,
            last_interaction: Self::now(),
        }
    }

    fn now() -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    pub fn update(&mut self, dt: f64) -> BehaviorAction {
        self.action_timer += dt;

        if matches!(self.current_action, BehaviorAction::WalkLeft | BehaviorAction::WalkRight) {
            self.energy = (self.energy - dt * 0.5).max(0.0);
        } else if matches!(self.current_action, BehaviorAction::Sleep) {
            self.energy = (self.energy + dt * 2.0).min(100.0);
        } else {
            self.energy = (self.energy - dt * 0.1).max(0.0);
        }

        self.boredom = (self.boredom + dt * 0.3).min(100.0);

        let time_since_interaction = Self::now() - self.last_interaction;
        self.happiness = if time_since_interaction > 60.0 {
            (self.happiness - dt * 0.5).max(0.0)
        } else {
            (self.happiness + dt * 0.2).min(100.0)
        };

        if self.action_timer >= self.action_duration {
            self.action_timer = 0.0;
            self.choose_next_action()
        } else {
            self.current_action.clone()
        }
    }

    fn choose_next_action(&mut self) -> BehaviorAction {
        let mut rng = rand::thread_rng();

        if self.energy < 20.0 {
            self.action_duration = rng.gen_range(5.0..15.0);
            self.current_action = BehaviorAction::Sleep;
            return BehaviorAction::Sleep;
        }

        if self.happiness > 70.0 && self.energy > 50.0 {
            let roll = rng.gen_range(0..100);
            if roll < 30 {
                self.action_duration = rng.gen_range(1.0..3.0);
                self.boredom = (self.boredom - 10.0).max(0.0);
                self.current_action = BehaviorAction::Jump;
                return BehaviorAction::Jump;
            } else if roll < 60 {
                let dir = if rng.gen_bool(0.5) {
                    BehaviorAction::WalkLeft
                } else {
                    BehaviorAction::WalkRight
                };
                self.action_duration = rng.gen_range(2.0..6.0);
                self.boredom = (self.boredom - 15.0).max(0.0);
                self.current_action = dir.clone();
                return dir;
            }
        }

        let roll = rng.gen_range(0..100);
        let action = if roll < 40 {
            BehaviorAction::Idle
        } else if roll < 60 {
            BehaviorAction::Sit
        } else if roll < 80 {
            if rng.gen_bool(0.5) {
                BehaviorAction::WalkLeft
            } else {
                BehaviorAction::WalkRight
            }
        } else {
            BehaviorAction::Idle
        };

        self.action_duration = rng.gen_range(2.0..5.0);
        self.current_action = action.clone();
        action
    }

    pub fn on_interact(&mut self) {
        self.last_interaction = Self::now();
        self.happiness = (self.happiness + 10.0).min(100.0);
        self.boredom = (self.boredom - 20.0).max(0.0);
        self.action_timer = self.action_duration;
    }

    pub fn pet_mood(&self) -> String {
        if self.energy < 20.0 {
            "疲倦".to_string()
        } else if self.happiness < 30.0 {
            "伤心".to_string()
        } else if self.happiness > 80.0 {
            "开心".to_string()
        } else if self.boredom > 70.0 {
            "饥饿".to_string()
        } else {
            "平静".to_string()
        }
    }
}
