struct Map {
    grid: Vec<Vec<i32>>, 
}

impl Map {
    fn generate(rows: usize, cols: usize) -> Self {
        let grid = vec![vec![0; cols]; rows];
        Map { grid }
    }

    fn fill(&mut self) {
        for row in &mut self.grid {
            for col in row {
                *col = 1;
            }
        }
    }

    fn set_tile(&mut self, x: usize, y: usize, value: i32) {
        if y < self.grid.len() && x < self.grid[0].len() {
            self.grid[y][x] = value;
        } else {
            println!("Ошибка: Координаты ({}, {}) вышли за границы карты!", x, y);
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<i32> {
        if y < self.grid.len() && x < self.grid[0].len() {
            Some(self.grid[y][x])
        } else {
            None
        }
    }

    fn print(&self) {
        for row in &self.grid {
            for col in row {
                print!("{} ", col);
            }
            println!();
        }
    }

    fn update(&mut self) {
        for row in &mut self.grid {
            for col in row {
                if *col==1 {
                    *col = 0;
                }
            }
        }
    }
}

fn main() {
    let mut my_map = Map::generate(10, 10);
    
    my_map.fill();  
    my_map.print();
    println!("\n");
    my_map.update();
    my_map.print();
    println!("\n");
    my_map.set_tile(5, 5, 1);
    my_map.print();
    println!("\n");
    my_map.update();
    my_map.print();

}

