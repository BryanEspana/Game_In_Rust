use crate::map::Map;
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub direction: f64,
    pub fov: f64,
}

impl Player {
    pub fn new(x: f64, y: f64, direction: f64) -> Self {
        Self {
            x,
            y,
            direction,
            fov: 60.0_f64.to_radians(),
        }
    }

    pub fn move_forward(&mut self, distance: f64, map: &Map) {
        let speed = 0.5;
        let new_x = self.x + self.direction.cos() * distance * speed;
        let new_y = self.y + self.direction.sin() * distance * speed;
        let min_distance_to_wall = 0.5;

        if !map.is_wall(new_x, self.y) && !map.is_wall(new_x + min_distance_to_wall, self.y) {
            self.x = new_x;
        }
        if !map.is_wall(self.x, new_y) && !map.is_wall(self.x, new_y + min_distance_to_wall) {
            self.y = new_y;
        }
    }

    pub fn move_backward(&mut self, distance: f64, map: &Map) {
        let speed = 0.5;
        let new_x = self.x - self.direction.cos() * distance * speed;
        let new_y = self.y - self.direction.sin() * distance * speed;
        let min_distance_to_wall = 0.5;

        if !map.is_wall(new_x, self.y) && !map.is_wall(new_x + min_distance_to_wall, self.y) {
            self.x = new_x;
        }
        if !map.is_wall(self.x, new_y) && !map.is_wall(self.x, new_y + min_distance_to_wall) {
            self.y = new_y;
        }
    }

    pub fn turn_left(&mut self, angle: f64) {
        self.direction -= angle;
    }

    pub fn turn_right(&mut self, angle: f64) {
        self.direction += angle;
    }

    pub fn update(&mut self, map: &Map, delta_time: f64) {
        let _ = delta_time;
        let _ = map;
        // Lógica de actualización si es necesario
    }
}
