use crate::map::Map;
use crate::player::Player;
use crate::framebuffer::Framebuffer;
use image::{DynamicImage, GenericImageView};

pub struct Raycaster;

impl Raycaster {
    pub fn new() -> Self {
        Raycaster
    }

    pub fn cast_rays(&self, map: &Map, player: &Player, framebuffer: &mut Framebuffer, wall_texture: &DynamicImage) {
        for x in 0..framebuffer.width {
            let camera_x = 2.0 * (x as f64) / (framebuffer.width as f64) - 1.0;
            let ray_dir = player.direction + player.fov / 2.0 * camera_x;
    
            // Cast ray and get correct perpendicular distance
            let (mut distance, hit_side, hit_x, hit_y) = self.cast_ray(map, player, ray_dir);
    
            // Avoid too small distances that can cause distortion
            if distance < 0.01 {
                distance = 0.01;
            }
    
            // Ensure distance is perpendicular, adjusting calculation based on hit_side
            if hit_side == 0 {
                distance *= (ray_dir - player.direction).cos(); // Adjust perpendicular distance
            }
    
            let wall_height = (framebuffer.height as f64 / distance) as usize;
            let start = (framebuffer.height / 2).saturating_sub(wall_height / 2);
            let end = (framebuffer.height / 2) + wall_height / 2;
    
            // Calculate exact position in texture
            let texture_x = if hit_side == 0 {
                (hit_y % 1.0 * wall_texture.width() as f64) as u32
            } else {
                (hit_x % 1.0 * wall_texture.width() as f64) as u32
            };
    
            for y in start..end {
                let texture_y = ((y as f64 - start as f64) / wall_height as f64 * wall_texture.height() as f64) as u32;
                let color = wall_texture.get_pixel(texture_x, texture_y).0;
                let color = ((color[0] as u32) << 16) | ((color[1] as u32) << 8) | (color[2] as u32);
                framebuffer.point(x, y, color);
            }
        }
    }

    fn cast_ray(
        &self,
        map: &Map,
        player: &Player,
        ray_dir: f64,
    ) -> (f64, i32, f64, f64) {
        let mut distance = 0.0;
        let mut step_size = 0.05;
        let mut hit_side = 0; // 0 for vertical, 1 for horizontal
        let mut current_x = player.x;
        let mut current_y = player.y;

        while !map.is_wall(current_x, current_y) && distance < 100.0 {
            let next_x = current_x + ray_dir.cos() * step_size;
            let next_y = current_y + ray_dir.sin() * step_size;

            if (next_x as usize) != (current_x as usize) {
                hit_side = 0;
            } else if (next_y as usize) != (current_y as usize) {
                hit_side = 1;
            }

            current_x = next_x;
            current_y = next_y;
            distance += step_size;

            // Adjust step size dynamically based on distance
            if distance > 50.0 {
                step_size = 0.1;
            } else if distance < 5.0 {
                step_size = 0.01; // High precision for short distances
            }
        }

        (distance, hit_side, current_x, current_y)
    }
}