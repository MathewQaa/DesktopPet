use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnimationFrame {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub duration_ms: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Animation {
    pub name: String,
    pub frames: Vec<AnimationFrame>,
    pub loop_animation: bool,
}

impl Animation {
    pub fn idle() -> Self {
        Animation {
            name: "idle".to_string(),
            frames: vec![
                AnimationFrame { x: 0, y: 0, width: 128, height: 128, duration_ms: 200 },
                AnimationFrame { x: 128, y: 0, width: 128, height: 128, duration_ms: 200 },
                AnimationFrame { x: 256, y: 0, width: 128, height: 128, duration_ms: 200 },
                AnimationFrame { x: 384, y: 0, width: 128, height: 128, duration_ms: 200 },
            ],
            loop_animation: true,
        }
    }

    pub fn walk() -> Self {
        Animation {
            name: "walk".to_string(),
            frames: vec![
                AnimationFrame { x: 0, y: 128, width: 128, height: 128, duration_ms: 120 },
                AnimationFrame { x: 128, y: 128, width: 128, height: 128, duration_ms: 120 },
                AnimationFrame { x: 256, y: 128, width: 128, height: 128, duration_ms: 120 },
                AnimationFrame { x: 384, y: 128, width: 128, height: 128, duration_ms: 120 },
            ],
            loop_animation: true,
        }
    }

    pub fn sleep() -> Self {
        Animation {
            name: "sleep".to_string(),
            frames: vec![
                AnimationFrame { x: 0, y: 256, width: 128, height: 128, duration_ms: 400 },
                AnimationFrame { x: 128, y: 256, width: 128, height: 128, duration_ms: 400 },
            ],
            loop_animation: true,
        }
    }

    pub fn drag() -> Self {
        Animation {
            name: "drag".to_string(),
            frames: vec![
                AnimationFrame { x: 0, y: 384, width: 128, height: 128, duration_ms: 100 },
            ],
            loop_animation: false,
        }
    }

    pub fn jump() -> Self {
        Animation {
            name: "jump".to_string(),
            frames: vec![
                AnimationFrame { x: 128, y: 384, width: 128, height: 128, duration_ms: 150 },
                AnimationFrame { x: 256, y: 384, width: 128, height: 128, duration_ms: 150 },
                AnimationFrame { x: 384, y: 384, width: 128, height: 128, duration_ms: 150 },
            ],
            loop_animation: false,
        }
    }

    pub fn sit() -> Self {
        Animation {
            name: "sit".to_string(),
            frames: vec![
                AnimationFrame { x: 0, y: 512, width: 128, height: 128, duration_ms: 200 },
                AnimationFrame { x: 128, y: 512, width: 128, height: 128, duration_ms: 200 },
            ],
            loop_animation: true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Animator {
    pub current_animation: String,
    pub current_frame: usize,
    pub frame_timer: f64,
}

impl Animator {
    pub fn new() -> Self {
        Animator {
            current_animation: "idle".to_string(),
            current_frame: 0,
            frame_timer: 0.0,
        }
    }

    pub fn set_animation(&mut self, name: &str) {
        if self.current_animation != name {
            self.current_animation = name.to_string();
            self.current_frame = 0;
            self.frame_timer = 0.0;
        }
    }

    pub fn get_frame(&self, animations: &[Animation]) -> Option<AnimationFrame> {
        animations.iter()
            .find(|a| a.name == self.current_animation)
            .and_then(|a| a.frames.get(self.current_frame).cloned())
    }
}
