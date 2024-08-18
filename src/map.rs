pub struct Map {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<char>>,
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
}

impl Map {
    pub fn new_level_1() -> Self {
        let grid = vec![
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', '#'],
            vec!['#', ' ', '#', '#', '#', ' ', '#', ' ', '#', '#'],
            vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'],
            vec!['#', '#', '#', ' ', '#', '#', ' ', '#', ' ', '#'],
            vec!['#', ' ', ' ', ' ', '#', ' ', ' ', '#', ' ', '#'],
            vec!['#', ' ', '#', ' ', '#', '#', ' ', '#', ' ', '#'],
            vec!['#', ' ', '#', ' ', ' ', ' ', ' ', ' ', ' ', '#'],
            vec!['#', ' ', ' ', ' ', '#', ' ', ' ', ' ', 'E', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
        ];

        Self {
            width: grid[0].len(),
            height: grid.len(),
            grid,
            start_x: 1.0,
            start_y: 1.0,
            end_x: 8.0,
            end_y: 8.0,
        }
    }

    pub fn new_level_2() -> Self {
        let grid = vec![
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'],
            vec!['#', ' ', '#', '#', '#', ' ', '#', '#', ' ', '#'],
            vec!['#', ' ', '#', ' ', ' ', ' ', '#', ' ', ' ', '#'],
            vec!['#', ' ', '#', ' ', '#', '#', ' ', '#', ' ', '#'],
            vec!['#', ' ', ' ', ' ', '#', ' ', ' ', ' ', ' ', '#'],
            vec!['#', '#', '#', ' ', '#', ' ', '#', '#', '#', '#'],
            vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'],
            vec!['#', ' ', ' ', ' ', '#', ' ', ' ', ' ', 'E', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
        ];

        Self {
            width: grid[0].len(),
            height: grid.len(),
            grid,
            start_x: 1.0,
            start_y: 1.0,
            end_x: 8.0,
            end_y: 8.0,
        }
    }
    pub fn is_wall(&self, x: f64, y: f64) -> bool {
        let xi = x.floor() as usize;
        let yi = y.floor() as usize;

        // Asegurarnos de que no nos salimos de los límites del mapa
        if xi >= self.width || yi >= self.height {
            return true;
        }

        self.grid[yi][xi] == '#'
    }

    pub fn is_end_position(&self, x: f64, y: f64) ->bool {
        (x.floor() as usize == self.end_x as usize) && (y.floor() as usize == self.end_y as usize)
    }
}
