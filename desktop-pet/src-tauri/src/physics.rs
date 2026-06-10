use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PetPhysics {
    pub position: Vec2,
    pub velocity: Vec2,
    pub gravity: f64,
    pub friction: f64,
    pub is_on_ground: bool,
    pub is_on_wall: bool,
    pub is_on_ceiling: bool,
    pub facing_right: bool,
    pub is_dragging: bool,
    pub drag_offset: Vec2,
}

impl PetPhysics {
    pub fn new(x: f64, y: f64) -> Self {
        PetPhysics {
            position: Vec2 { x, y },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            gravity: 0.5,
            friction: 0.85,
            is_on_ground: false,
            is_on_wall: false,
            is_on_ceiling: false,
            facing_right: true,
            is_dragging: false,
            drag_offset: Vec2 { x: 0.0, y: 0.0 },
        }
    }

    pub fn update(&mut self, screen_width: f64, screen_height: f64, pet_size: f64) {
        if self.is_dragging {
            return;
        }

        // 应用重力
        if !self.is_on_ground {
            self.velocity.y += self.gravity;
        }

        // 应用摩擦力
        self.velocity.x *= self.friction;

        // 更新位置
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        // 地面碰撞
        let ground_y = screen_height - pet_size;
        if self.position.y >= ground_y {
            self.position.y = ground_y;
            self.velocity.y = 0.0;
            self.is_on_ground = true;
        } else {
            self.is_on_ground = false;
        }

        // 天花板碰撞
        if self.position.y <= 0.0 {
            self.position.y = 0.0;
            self.velocity.y = 0.0;
            self.is_on_ceiling = true;
        } else {
            self.is_on_ceiling = false;
        }

        // 墙壁碰撞
        if self.position.x <= 0.0 {
            self.position.x = 0.0;
            self.velocity.x = self.velocity.x.abs();
            self.is_on_wall = true;
            self.facing_right = true;
        } else if self.position.x >= screen_width - pet_size {
            self.position.x = screen_width - pet_size;
            self.velocity.x = -self.velocity.x.abs();
            self.is_on_wall = true;
            self.facing_right = false;
        } else {
            self.is_on_wall = false;
        }

        // 更新朝向
        if self.velocity.x > 0.1 {
            self.facing_right = true;
        } else if self.velocity.x < -0.1 {
            self.facing_right = false;
        }
    }

    pub fn start_drag(&mut self, mouse_x: f64, mouse_y: f64) {
        self.is_dragging = true;
        self.drag_offset = Vec2 {
            x: mouse_x - self.position.x,
            y: mouse_y - self.position.y,
        };
        self.velocity = Vec2 { x: 0.0, y: 0.0 };
    }

    pub fn update_drag(&mut self, mouse_x: f64, mouse_y: f64) {
        if self.is_dragging {
            self.position.x = mouse_x - self.drag_offset.x;
            self.position.y = mouse_y - self.drag_offset.y;
        }
    }

    pub fn end_drag(&mut self, velocity_x: f64, velocity_y: f64) {
        self.is_dragging = false;
        self.velocity.x = velocity_x * 0.3;
        self.velocity.y = velocity_y * 0.3;
    }

    pub fn walk(&mut self, direction: f64) {
        if !self.is_dragging {
            self.velocity.x += direction * 0.5;
            self.velocity.x = self.velocity.x.clamp(-5.0, 5.0);
        }
    }

    pub fn jump(&mut self) {
        if self.is_on_ground && !self.is_dragging {
            self.velocity.y = -12.0;
            self.is_on_ground = false;
        }
    }

    pub fn climb_wall(&mut self) {
        if self.is_on_wall && !self.is_dragging {
            self.velocity.y = -3.0;
        }
    }
}
