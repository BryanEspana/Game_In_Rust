use image::{DynamicImage, GenericImageView};

use crate::framebuffer::Framebuffer;
use crate::map::Map;
use crate::player::Player;
use crate::raycaster::Raycaster;

pub struct Renderer {
    pub framebuffer: Framebuffer,
    pub raycaster: Raycaster,
    pub wall_texture: DynamicImage,
    pub floor_texture: DynamicImage,
    pub sky_color: u32,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let wall_texture = image::open("src/pared.png").unwrap();
        let floor_texture = image::open("src/tierra.png").unwrap();
        let sky_color = 0x87CEEB; // Color azul cielo

        Self {
            framebuffer: Framebuffer::new(width, height),
            raycaster: Raycaster::new(),
            wall_texture,
            floor_texture,
            sky_color,
        }
    }

    pub fn render_scene(&mut self, map: &Map, player: &Player) {
        self.framebuffer.clear(self.sky_color);
        self.render_floor(player);
        self.raycaster.cast_rays(map, player, &mut self.framebuffer, &self.wall_texture);
        self.render_goal_3d(map, player);
        self.render_minimap(map, player);
    }

    fn render_goal_3d(&mut self, map: &Map, player: &Player) {
        let goal_x = map.end_x;
        let goal_y = map.end_y;
    
        let dx = goal_x - player.x;
        let dy = goal_y - player.y;
        let distance = (dx * dx + dy * dy).sqrt();
    
        if distance < 0.1 {
            return;
        }
    
        let angle_to_goal = (dy).atan2(dx) - player.direction;
    
        if angle_to_goal.abs() < player.fov / 2.0 {
            let screen_x = (self.framebuffer.width as f64 / 2.0) * (1.0 + angle_to_goal / player.fov);
            let goal_height = (self.framebuffer.height as f64 / distance) as usize;
            let start_y = (self.framebuffer.height / 2).saturating_sub(goal_height / 2); // Evitar desbordamientos
            let end_y = start_y + goal_height;
    
            if screen_x >= 0.0 && screen_x < self.framebuffer.width as f64 {
                let start_x = (screen_x as usize).saturating_sub(goal_height / 2); // Evita el desbordamiento
                let end_x = (screen_x as usize).saturating_add(goal_height / 2); // Asegura que no haya desbordamiento al sumar
            
                for y in start_y..end_y {
                    for x in start_x..end_x {
                        if x < self.framebuffer.width && y < self.framebuffer.height {
                            self.framebuffer.point(x, y, 0x00FF00); // Color verde para la meta
                        }
                    }
                }
            }
            
        }
    }
    

    

    fn render_floor(&mut self, player: &Player) {
        let height = self.framebuffer.height;
        let center = height / 2;
    
        // Pre-calcular algunas constantes
        let floor_texture_width = self.floor_texture.width() as f64;
        let floor_texture_height = self.floor_texture.height() as f64;
    
        for y in center..height {
            // Calcular la distancia a este nivel del piso
            let row_distance = (self.framebuffer.height as f64 / (2.0 * (y - center) as f64)) as f64;
            
            // Calcular las coordenadas del piso basadas en la dirección del jugador
            let floor_step_x = row_distance * (player.direction + player.fov / 2.0).cos();
            let floor_step_y = row_distance * (player.direction + player.fov / 2.0).sin();
    
            let mut floor_x = player.x + floor_step_x;
            let mut floor_y = player.y + floor_step_y;
    
            for x in 0..self.framebuffer.width {
                // Mapear las coordenadas del piso a las coordenadas de la textura
                let texture_x = ((floor_x % 1.0) * floor_texture_width) as u32;
                let texture_y = ((floor_y % 1.0) * floor_texture_height) as u32;
    
                // Obtener el color de la textura y renderizar el píxel en el framebuffer
                let color = self.floor_texture.get_pixel(texture_x, texture_y).0;
                let color = ((color[0] as u32) << 16) | ((color[1] as u32) << 8) | (color[2] as u32); // Convertir a formato RGB
                self.framebuffer.point(x, y, color);
    
                // Actualizar las coordenadas del piso para el siguiente píxel
                floor_x += floor_step_x;
                floor_y += floor_step_y;
            }
        }
    }
    

    fn render_minimap(&mut self, map: &Map, player: &Player) {
        // Dibujar el minimapa en la esquina superior izquierda
        let minimap_scale = 4;

        for y in 0..map.height {
            for x in 0..map.width {
                let color = if map.is_wall(x as f64, y as f64) {
                    0xFFFFFF // Color de las paredes en el minimapa
                } else {
                    0x000000 // Color del suelo en el minimapa
                };

                for py in 0..minimap_scale {
                    for px in 0..minimap_scale {
                        self.framebuffer.point(x * minimap_scale + px, y * minimap_scale + py, color);
                    }
                }
            }
        }

        // Dibujar la posición del jugador en el minimapa
        let player_x = (player.x * minimap_scale as f64) as usize;
        let player_y = (player.y * minimap_scale as f64) as usize;
        for py in 0..minimap_scale {
            for px in 0..minimap_scale {
                self.framebuffer.point(player_x + px, player_y + py, 0xFF0000); // Rojo para la posición del jugador
            }
        }
    }
}
