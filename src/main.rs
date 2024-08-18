mod framebuffer;
mod map;
mod player;
mod raycaster;
mod renderer;

use crate::map::Map;
use crate::player::Player;
use crate::renderer::Renderer;

use minifb::{Key, Window, WindowOptions, MouseButton};



// Estado del juego: pantalla de bienvenida, nivel 1, nivel 2
pub enum GameState {
    WelcomeScreen,
    Level1,
    Level2,
    SuccessScreen,
}

pub struct Game {
    width: usize,
    height: usize,
    window: Window,
    renderer: Renderer,
    player: Player,
    map: Map,
    target_fps: usize,
    state: GameState, // Estado del juego
}

impl Game {

    fn check_goal_reached(&self, goal_x: f64, goal_y: f64, player_x: f64, player_y: f64) -> bool {
        let dx = goal_x - player_x;
        let dy = goal_y - player_y;
        let distance = (dx * dx + dy * dy).sqrt();
    
        // Si el jugador está lo suficientemente cerca de la meta
        distance < 0.5
    }

    pub fn new(width: usize, height: usize) -> Self {
        let window = Window::new(
            "3D Raycaster",
            width,
            height,
            WindowOptions::default(),
        )
        .unwrap();

        let renderer = Renderer::new(width, height);
        let map = Map::new_level_1(); // Inicializa con el mapa del nivel 1
        let player = Player::new(3.0, 3.0, 0.0);

        Self {
            width,
            height,
            window,
            renderer,
            player,
            map,
            target_fps: 160,
            state: GameState::WelcomeScreen, // Inicia en la pantalla de bienvenida
        }
    }

    pub fn run(&mut self) {
        // Duración de cada frame en segundos
        let frame_duration = std::time::Duration::from_secs_f64(1.0 / 60.0);
        let mut frame_count = 0;
        let mut fps_timer = std::time::Instant::now();
    
        // Bucle principal del juego
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // Actualizar el estado del juego según el estado actual
            match self.state {
                GameState::WelcomeScreen => {
                    self.show_welcome_screen();
                }
                GameState::Level1 => self.run_level(1),
                GameState::Level2 => self.run_level(2),
                GameState::SuccessScreen => self.show_success_screen(), // Pantalla de éxito
            }
    
            // Calcular los frames por segundo (FPS)
            frame_count += 1;
            if fps_timer.elapsed() >= std::time::Duration::from_secs(1) {
                let fps = frame_count;
                frame_count = 0;
                fps_timer = std::time::Instant::now();
                self.window.set_title(&format!("3D Raycaster - FPS: {}", fps));
            }
    
            // Esperar el tiempo necesario para mantener una tasa de refresco de 60 FPS
            std::thread::sleep(frame_duration);
        }
    }

    fn show_welcome_screen(&mut self) {
        let framebuffer = &mut self.renderer.framebuffer;
    
        // Limpiar el framebuffer una sola vez
        framebuffer.clear(0x000000); // Fondo negro
    
        // Dibujar todo el contenido de la pantalla de bienvenida
        self.draw_text(100, 50, "WELCOME SELECCIONE UN NIVEL", 0xFFFFFF); // Dibujar "WELCOME"
        self.draw_button(100, 150, 200, 50, "1", 0xFFFFFF, 0x007BFF); // Botón Level 1
        self.draw_button(100, 250, 200, 50, "2", 0xFFFFFF, 0xFF5722); // Botón Level 2
    
        // Verificar si se hizo clic en algún botón **antes de actualizar el framebuffer**
        if let Some((mouse_x, mouse_y)) = self.window.get_mouse_pos(minifb::MouseMode::Clamp) {
            if self.window.get_mouse_down(MouseButton::Left) {
                if self.is_inside_button(mouse_x, mouse_y, 100, 150, 200, 50) {
                    self.map = Map::new_level_1(); // Cargar mapa del nivel 1
                    self.state = GameState::Level1;
                } else if self.is_inside_button(mouse_x, mouse_y, 100, 250, 200, 50) {
                    self.map = Map::new_level_2(); // Cargar mapa del nivel 2
                    self.state = GameState::Level2;
                }
            }
        }
    
        // Actualizar el buffer en la ventana solo después de procesar los clics
        self.window.update_with_buffer(&self.renderer.framebuffer.buffer, self.width, self.height).unwrap();
    }
    
    

        fn draw_text(&mut self, x: usize, y: usize, text: &str, color: u32) {
            let font = vec![
                // A-Z representados en un formato de 5x7 píxeles
                // A
                [
                    " 000 ",
                    "0   0",
                    "0   0",
                    "00000",
                    "0   0",
                    "0   0",
                    "0   0",
                ],
                // B
                [
                    "0000 ",
                    "0   0",
                    "0000 ",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0000 ",
                ],
                // C
                [
                    " 0000",
                    "0    ",
                    "0    ",
                    "0    ",
                    "0    ",
                    "0    ",
                    " 0000",
                ],
                // D
                [
                    "0000 ",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0000 ",
                ],
                // E
                [
                    "00000",
                    "0    ",
                    "0000 ",
                    "0    ",
                    "0    ",
                    "0    ",
                    "00000",
                ],
                // F
                [
                    "00000",
                    "0    ",
                    "0000 ",
                    "0    ",
                    "0    ",
                    "0    ",
                    "0    ",
                ],
                // G
                [
                    " 0000",
                    "0    ",
                    "0  00",
                    "0   0",
                    "0   0",
                    "0   0",
                    " 0000",
                ],
                // H
                [
                    "0   0",
                    "0   0",
                    "00000",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0   0",
                ],
                // I
                [
                    " 000 ",
                    "  0  ",
                    "  0  ",
                    "  0  ",
                    "  0  ",
                    "  0  ",
                    " 000 ",
                ],
                // J
                [
                    "   00",
                    "    0",
                    "    0",
                    "    0",
                    "0   0",
                    "0   0",
                    " 000 ",
                ],
                // K
                [
                    "0   0",
                    "0  0 ",
                    "000  ",
                    "0 0  ",
                    "0  0 ",
                    "0   0",
                    "0   0",
                ],
                // L
                [
                    "0    ",
                    "0    ",
                    "0    ",
                    "0    ",
                    "0    ",
                    "0    ",
                    "00000",
                ],
                // M
                [
                    "0   0",
                    "00 00",
                    "0 0 0",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0   0",
                ],
                // N
                [
                    "0   0",
                    "00  0",
                    "0 0 0",
                    "0  00",
                    "0   0",
                    "0   0",
                    "0   0",
                ],
                // O
                [
                    " 000 ",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0   0",
                    " 000 ",
                ],
                // P
                [
                    "0000 ",
                    "0   0",
                    "0   0",
                    "0000 ",
                    "0    ",
                    "0    ",
                    "0    ",
                ],
                // Q
                [
                    " 000 ",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0 0 0",
                    "0  00",
                    " 0000",
                ],
                // R
                [
                    "0000 ",
                    "0   0",
                    "0   0",
                    "0000 ",
                    "0  0 ",
                    "0   0",
                    "0   0",
                ],
                // S
                [
                    " 0000",
                    "0    ",
                    "0    ",
                    " 000 ",
                    "    0",
                    "    0",
                    "0000 ",
                ],
                // T
                [
                    "00000",
                    "  0  ",
                    "  0  ",
                    "  0  ",
                    "  0  ",
                    "  0  ",
                    "  0  ",
                ],
                // U
                [
                    "0   0",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0   0",
                    " 000 ",
                ],
                // V
                [
                    "0   0",
                    "0   0",
                    "0   0",
                    "0   0",
                    "0   0",
                    " 0 0 ",
                    "  0  ",
                ],
                // W
                [
                    "0   0",
                    "0   0",
                    "0   0",
                    "0 0 0",
                    "0 0 0",
                    "0 0 0",
                    " 0 0 ",
                ],
                // X
                [
                    "0   0",
                    "0   0",
                    " 0 0 ",
                    "  0  ",
                    " 0 0 ",
                    "0   0",
                    "0   0",
                ],
                // Y
                [
                    "0   0",
                    "0   0",
                    " 0 0 ",
                    "  0  ",
                    "  0  ",
                    "  0  ",
                    "  0  ",
                ],
                // Z
                [
                    "00000",
                    "    0",
                    "   0 ",
                    "  0  ",
                    " 0   ",
                    "0    ",
                    "00000",
                ],
                // 1
                [
                    "  0  ",
                    " 00  ",
                    "  0  ",
                    "  0  ",
                    "  0  ",
                    "  0  ",
                    " 000 ",
                ],
                // 2
                [
                    " 000 ",
                    "0   0",
                    "    0",
                    "   0 ",
                    "  0  ",
                    " 0   ",
                    "00000",
                ],
            ];
    
            let scale = 2; // Escala de los caracteres
    let char_width = 5; // Ancho de cada carácter en la fuente
    let char_height = 7; // Altura de cada carácter en la fuente

    for (i, c) in text.chars().enumerate() {
        if let Some(font_index) = match c {
            'A'..='Z' => Some((c as u8 - b'A') as usize),
            '1' => Some(26), // Índice del número 1
            '2' => Some(27), // Índice del número 2
            _ => None,
        } {
            let letter = &font[font_index];
            for (row, line) in letter.iter().enumerate() {
                for (col, ch) in line.chars().enumerate() {
                    if ch == '0' {
                        for sx in 0..scale {
                            for sy in 0..scale {
                                self.renderer.framebuffer.point(
                                    x + i * (char_width + 1) * scale + col * scale + sx,
                                    y + row * scale + sy,
                                    color,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
        }
    
        fn draw_button(&mut self, x: usize, y: usize, width: usize, height: usize, text: &str, text_color: u32, background_color: u32) {
            let framebuffer = &mut self.renderer.framebuffer;
        
            // Dibujar el fondo del botón
            for py in 0..height {
                for px in 0..width {
                    framebuffer.point(x + px, y + py, background_color);
                }
            }
        
            // Dibujar el texto del botón
            self.draw_text(x + 20, y + 15, text, text_color); // Ajustar la posición para centrar el texto
        }
        

    fn is_inside_button(&self, mouse_x: f32, mouse_y: f32, button_x: usize, button_y: usize, button_width: usize, button_height: usize) -> bool {
        mouse_x >= button_x as f32
            && mouse_x <= (button_x + button_width) as f32
            && mouse_y >= button_y as f32
            && mouse_y <= (button_y + button_height) as f32
    }

    pub fn run_level(&mut self, level: usize) {
        let mut last_frame_time = std::time::Instant::now();
        let frame_duration = std::time::Duration::from_secs_f64(1.0 / 60.0);
        let mut goal_reached_flag = false; // Bandera para determinar si la meta fue alcanzada
    
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let now = std::time::Instant::now();
            let delta_time = now.duration_since(last_frame_time).as_secs_f64();
            last_frame_time = now;
    
            // Si la meta ha sido alcanzada, evitamos seguir procesando el nivel
            if goal_reached_flag {
                std::thread::sleep(std::time::Duration::from_millis(100));
                self.state = GameState::SuccessScreen;
                break;
            }
    
            // Obtener las coordenadas de la meta y del jugador
            let goal_x = self.map.end_x;
            let goal_y = self.map.end_y;
            let player_x = self.player.x;
            let player_y = self.player.y;
    
            // Verificar si se ha alcanzado la meta
            let goal_reached = self.check_goal_reached(goal_x, goal_y, player_x, player_y);
    
            // Si se alcanzó la meta, establecemos la bandera
            if goal_reached {
                goal_reached_flag = true;
                continue; // Saltar el resto del ciclo actual
            }
    
            // Actualizar el estado del juego
            self.handle_input();
            self.player.update(&self.map, delta_time);
            self.renderer.render_scene(&self.map, &self.player);
            self.window.update_with_buffer(&self.renderer.framebuffer.buffer, self.width, self.height).unwrap();
    
            let frame_elapsed = now.elapsed();
            if frame_elapsed < frame_duration {
                std::thread::sleep(frame_duration - frame_elapsed);
            }
        }
    }
    
    
    
    
    fn handle_input(&mut self) {
        if self.window.is_key_down(Key::W) || self.window.is_key_down(Key::Up) {
            self.player.move_forward(0.1, &self.map);
        }
        if self.window.is_key_down(Key::S) || self.window.is_key_down(Key::Down) {
            self.player.move_backward(0.1, &self.map);
        }
        if self.window.is_key_down(Key::A) || self.window.is_key_down(Key::Left) {
            self.player.turn_left(0.05);
        }
        if self.window.is_key_down(Key::D) || self.window.is_key_down(Key::Right) {
            self.player.turn_right(0.05);
        }
    }

    pub fn start_level(&mut self, level: usize) {
        match level {
            1 => {
                self.map = Map::new_level_1();
                self.player.x = self.map.start_x;
                self.player.y = self.map.start_y;
                self.state = GameState::Level1;
            }
            2 => {
                self.map = Map::new_level_2();
                self.player.x = self.map.start_x;
                self.player.y = self.map.start_y;
                self.state = GameState::Level2;
            }
            _ => {} // Handle invalid levels (optional)
        }
    }
    pub fn show_success_screen(&mut self) {
        let framebuffer = &mut self.renderer.framebuffer;
    
        // Clear the framebuffer
        framebuffer.clear(0x000000); // Black background
    
        // Draw success screen content
        self.draw_text(100, 50, "¡FELICIDADES! NIVEL COMPLETADO", 0xFFFFFF);
        self.draw_button(100, 150, 200, 50, "1", 0xFFFFFF, 0x007BFF); // Level 1 Button
        self.draw_button(100, 250, 200, 50, "2", 0xFFFFFF, 0xFF5722); // Level 2 Button
    
        // Check if any button was clicked
        if let Some((mouse_x, mouse_y)) = self.window.get_mouse_pos(minifb::MouseMode::Clamp) {
            if self.window.get_mouse_down(MouseButton::Left) {
                if self.is_inside_button(mouse_x, mouse_y, 100, 150, 200, 50) {
                    self.map = Map::new_level_1(); // Load level 1
                    self.player.x = self.map.start_x;
                    self.player.y = self.map.start_y;
                    self.state = GameState::Level1;
                } else if self.is_inside_button(mouse_x, mouse_y, 100, 250, 200, 50) {
                    self.map = Map::new_level_2(); // Load level 2
                    self.player.x = self.map.start_x;
                    self.player.y = self.map.start_y;
                    self.state = GameState::Level2;
                }
            }
        }
    
        // Update window buffer
        self.window.update_with_buffer(&self.renderer.framebuffer.buffer, self.width, self.height).unwrap();
    }
}

fn main() {
    let mut game = Game::new(640, 480);
    game.run();
}
