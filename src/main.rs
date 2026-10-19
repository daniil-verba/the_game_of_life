// Тип границы мира
enum BoundaryType {
    Bounded,    // Ограниченный мир (за границей - мёртвые клетки)
    Toroidal,   // Циклический мир (тор)
}

struct Map {
    grid: Vec<Vec<i32>>,
    boundary: BoundaryType,
}

impl Map {
    fn generate(rows: usize, cols: usize, boundary: BoundaryType) -> Self {
        let grid = vec![vec![0; cols]; rows];
        Map { grid, boundary }
    }

    fn set_tile(&mut self, x: usize, y: usize, value: i32) {
        if y < self.grid.len() && x < self.grid[0].len() {
            self.grid[y][x] = value;
        } else {
            println!("Ошибка: Координаты ({}, {}) вышли за границы карты!", x, y);
        }
    }

    fn print(&self) {
        for row in &self.grid {
            for col in row {
                if *col == 1 {
                    print!("█ ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }

    // Подсчёт соседей с учётом типа границы
    fn count_neighbors(&self, x: usize, y: usize) -> i32 {
        let mut count = 0;
        let rows = self.grid.len();
        let cols = self.grid[0].len();

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let (nx, ny) = match self.boundary {
                    BoundaryType::Toroidal => {
                        // Циклические координаты (тор)
                        let nx = (x as i32 + dx + cols as i32) % cols as i32;
                        let ny = (y as i32 + dy + rows as i32) % rows as i32;
                        (nx as usize, ny as usize)
                    }
                    BoundaryType::Bounded => {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        // Если вышли за границы - клетка мертва, пропускаем
                        if nx < 0 || nx >= cols as i32 || ny < 0 || ny >= rows as i32 {
                            continue;
                        }
                        (nx as usize, ny as usize)
                    }
                };

                if self.grid[ny][nx] == 1 {
                    count += 1;
                }
            }
        }
        count
    }

    fn update(&mut self) {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let mut next_grid = vec![vec![0; cols]; rows];

        for y in 0..rows {
            for x in 0..cols {
                let neighbors = self.count_neighbors(x, y);
                let current = self.grid[y][x];

                if current == 1 {
                    if neighbors == 2 || neighbors == 3 {
                        next_grid[y][x] = 1;
                    }
                } else {
                    if neighbors == 3 {
                        next_grid[y][x] = 1;
                    }
                }
            }
        }

        self.grid = next_grid;
    }

    fn fill_random(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};

        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut rng = seed as u32;

        for row in &mut self.grid {
            for col in row {
                rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
                *col = if (rng / 65536) % 100 < 30 { 1 } else { 0 };
            }
        }
    }

    // Создаём глайдер в заданной позиции
    fn place_glider(&mut self, x: usize, y: usize) {
        let glider = [
            (1, 0), (2, 1), (0, 2), (1, 2), (2, 2),
        ];
        for (dx, dy) in glider {
            self.set_tile(x + dx, y + dy, 1);
        }
    }
}

fn main() {
    // === Режим 1: Ограниченный мир ===
    println!("=== Ограниченный мир (Bounded) ===");
    println!("Глайдер движется к краю и 'разбивается' о стену\n");
    
    let mut bounded_map = Map::generate(15, 30, BoundaryType::Bounded);
    bounded_map.place_glider(2, 2);
    
    for generation in 0..=15 {
        println!("Поколение {}:", generation);
        bounded_map.print();
        println!();
        bounded_map.update();
    }

    // === Режим 2: Циклический мир (тор) ===
    println!("\n=== Циклический мир (Toroidal) ===");
    println!("Глайдер проходит через край и появляется с другой стороны\n");
    
    let mut toroidal_map = Map::generate(15, 30, BoundaryType::Toroidal);
    toroidal_map.place_glider(2, 2);
    
    for generation in 0..=100{
        println!("Поколение {}:", generation);
        toroidal_map.print();
        println!();
        toroidal_map.update();
    }
}
