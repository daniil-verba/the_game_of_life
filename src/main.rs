use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum BoundaryType {
    Bounded,
    Toroidal,
}

struct Map {
    grid: Vec<Vec<i32>>,
    boundary: BoundaryType,
    generation: u64,
}

impl Map {
    fn generate(rows: usize, cols: usize, boundary: BoundaryType) -> Self {
        let grid = vec![vec![0; cols]; rows];
        Map { grid, boundary, generation: 0 }
    }

    fn set_tile(&mut self, x: usize, y: usize, value: i32) {
        if y < self.grid.len() && x < self.grid[0].len() {
            self.grid[y][x] = value;
        }
    }

    fn toggle_tile(&mut self, x: usize, y: usize) {
        if y < self.grid.len() && x < self.grid[0].len() {
            self.grid[y][x] = if self.grid[y][x] == 1 { 0 } else { 1 };
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> i32 {
        let mut count = 0;
        let rows = self.grid.len();
        let cols = self.grid[0].len();

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }

                let (nx, ny) = match self.boundary {
                    BoundaryType::Toroidal => {
                        let nx = (x as i32 + dx + cols as i32) % cols as i32;
                        let ny = (y as i32 + dy + rows as i32) % rows as i32;
                        (nx as usize, ny as usize)
                    }
                    BoundaryType::Bounded => {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx < 0 || nx >= cols as i32 || ny < 0 || ny >= rows as i32 {
                            continue;
                        }
                        (nx as usize, ny as usize)
                    }
                };

                if self.grid[ny][nx] == 1 { count += 1; }
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

                next_grid[y][x] = if current == 1 {
                    if neighbors == 2 || neighbors == 3 { 1 } else { 0 }
                } else {
                    if neighbors == 3 { 1 } else { 0 }
                };
            }
        }

        self.grid = next_grid;
        self.generation += 1;
    }

    fn clear(&mut self) {
        for row in &mut self.grid {
            for col in row { *col = 0; }
        }
        self.generation = 0;
    }

    fn fill_random(&mut self) {
        for row in &mut self.grid {
            for col in row {
                *col = if rand::gen_range(0, 100) < 30 { 1 } else { 0 };
            }
        }
        self.generation = 0;
    }

    fn alive_count(&self) -> usize {
        self.grid.iter().flatten().filter(|&&c| c == 1).count()
    }

    // === Patterns ===

    fn place_glider(&mut self, x: usize, y: usize) {
        let pattern = [(1,0),(2,1),(0,2),(1,2),(2,2)];
        for (dx, dy) in pattern { self.set_tile(x+dx, y+dy, 1); }
    }

    fn place_pulsar(&mut self, x: usize, y: usize) {
        let pattern = [
            (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
            (0,2),(5,2),(7,2),(12,2),
            (0,3),(5,3),(7,3),(12,3),
            (0,4),(5,4),(7,4),(12,4),
            (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),
            (2,7),(3,7),(4,7),(8,7),(9,7),(10,7),
            (0,8),(5,8),(7,8),(12,8),
            (0,9),(5,9),(7,9),(12,9),
            (0,10),(5,10),(7,10),(12,10),
            (2,12),(3,12),(4,12),(8,12),(9,12),(10,12),
        ];
        for (dx, dy) in pattern { self.set_tile(x+dx, y+dy, 1); }
    }

    fn place_gosper_gun(&mut self, x: usize, y: usize) {
        let pattern = [
            (0,4),(0,5),(1,4),(1,5),
            (10,4),(10,5),(10,6),(11,3),(11,7),(12,2),(12,8),(13,2),(13,8),
            (14,5),(15,3),(15,7),(16,4),(16,5),(16,6),(17,5),
            (20,2),(20,3),(20,4),(21,2),(21,3),(21,4),(22,1),(22,5),
            (24,0),(24,1),(24,5),(24,6),
            (34,2),(34,3),(35,2),(35,3),
        ];
        for (dx, dy) in pattern { self.set_tile(x+dx, y+dy, 1); }
    }
}

// === Camera ===

struct Camera {
    x: f32,
    y: f32,
    zoom: f32,
}

impl Camera {
    fn new() -> Self { Camera { x: 0.0, y: 0.0, zoom: 1.0 } }

    fn screen_to_world(&self, sx: f32, sy: f32) -> (f32, f32) {
        ((sx - self.x) / self.zoom, (sy - self.y) / self.zoom)
    }
}

// === Main loop ===

#[macroquad::main("Game of Life")]
async fn main() {
    let rows = 100;
    let cols = 150;
    let mut map = Map::generate(rows, cols, BoundaryType::Toroidal);
    map.place_gosper_gun(2, 2);

    let mut camera = Camera::new();
    let mut cell_size: f32 = 8.0;
    let mut paused = true;
    let mut speed: f32 = 10.0; // generations per second
    let mut accumulator: f32 = 0.0;
    let mut show_help = true;

    loop {
        let dt = get_frame_time();

        // === Input handling ===

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let (wx, wy) = camera.screen_to_world(mx, my);
            let cx = (wx / cell_size) as i32;
            let cy = (wy / cell_size) as i32;
            if cx >= 0 && cy >= 0 {
                map.toggle_tile(cx as usize, cy as usize);
            }
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if is_key_pressed(KeyCode::N) {
            map.update();
        }

        if is_key_pressed(KeyCode::R) {
            map.clear();
        }

        if is_key_pressed(KeyCode::F) {
            map.fill_random();
        }

        if is_key_pressed(KeyCode::B) {
            map.boundary = match map.boundary {
                BoundaryType::Bounded => BoundaryType::Toroidal,
                BoundaryType::Toroidal => BoundaryType::Bounded,
            };
        }

        if is_key_pressed(KeyCode::Key1) {
            map.clear();
            map.place_glider(5, 5);
        }
        if is_key_pressed(KeyCode::Key2) {
            map.clear();
            map.place_pulsar(10, 10);
        }
        if is_key_pressed(KeyCode::Key3) {
            map.clear();
            map.place_gosper_gun(2, 2);
        }

        if is_key_pressed(KeyCode::H) {
            show_help = !show_help;
        }

        if is_key_down(KeyCode::Up) { speed = (speed * 1.05).min(100.0); }
        if is_key_down(KeyCode::Down) { speed = (speed / 1.05).max(0.5); }

        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            let factor = if wheel_y > 0.0 { 1.1 } else { 1.0 / 1.1 };
            let (mx, my) = mouse_position();
            let wx = (mx - camera.x) / camera.zoom;
            let wy = (my - camera.y) / camera.zoom;
            camera.zoom *= factor;
            camera.x = mx - wx * camera.zoom;
            camera.y = my - wy * camera.zoom;
        }

        if is_mouse_button_down(MouseButton::Middle) || is_mouse_button_down(MouseButton::Right) {
            let delta = mouse_delta_position();
            camera.x += delta.x;
            camera.y += delta.y;
        }

        // === Simulation ===
        if !paused {
            accumulator += dt * speed;
            while accumulator >= 1.0 {
                map.update();
                accumulator -= 1.0;
            }
        }

        // === Rendering ===
        clear_background(Color::new(0.08, 0.08, 0.12, 1.0));

        for y in 0..map.grid.len() {
            for x in 0..map.grid[0].len() {
                let sx = camera.x + x as f32 * cell_size * camera.zoom;
                let sy = camera.y + y as f32 * cell_size * camera.zoom;
                let size = cell_size * camera.zoom - 1.0;

                if sx + size < 0.0 || sy + size < 0.0 || sx > screen_width() || sy > screen_height() {
                    continue;
                }

                let color = if map.grid[y][x] == 1 {
                    Color::new(0.4, 1.0, 0.6, 1.0)
                } else {
                    Color::new(0.15, 0.15, 0.2, 1.0)
                };

                draw_rectangle(sx, sy, size, size, color);
            }
        }

        // === UI ===
        let bg_color = Color::new(0.0, 0.0, 0.0, 0.7);
        draw_rectangle(10.0, 10.0, 300.0, 130.0, bg_color);

        let boundary_str = match map.boundary {
            BoundaryType::Bounded => "Bounded (Edges)",
            BoundaryType::Toroidal => "Toroidal (Wrap)",
        };

        draw_text(&format!("Generation: {}", map.generation), 20.0, 35.0, 22.0, WHITE);
        draw_text(&format!("Alive: {}", map.alive_count()), 20.0, 60.0, 22.0, WHITE);
        draw_text(&format!("Boundary: {}", boundary_str), 20.0, 85.0, 22.0, YELLOW);
        draw_text(&format!("Speed: {:.1} gen/s", speed), 20.0, 110.0, 22.0, WHITE);
        draw_text(if paused { "⏸ PAUSED" } else { "▶ RUNNING" }, 20.0, 135.0, 22.0, 
                  if paused { RED } else { GREEN });

        if show_help {
            let help_y = screen_height() - 180.0;
            draw_rectangle(10.0, help_y, 400.0, 170.0, bg_color);
            draw_text("Controls:", 20.0, help_y + 25.0, 22.0, YELLOW);
            draw_text("LMB: Toggle cell", 20.0, help_y + 50.0, 18.0, WHITE);
            draw_text("RMB/MMB: Pan camera", 20.0, help_y + 70.0, 18.0, WHITE);
            draw_text("Scroll: Zoom", 20.0, help_y + 90.0, 18.0, WHITE);
            draw_text("Space: Pause, N: Step, R: Clear", 20.0, help_y + 110.0, 18.0, WHITE);
            draw_text("F: Random, B: Toggle boundary", 20.0, help_y + 130.0, 18.0, WHITE);
            draw_text("1/2/3: Glider/Pulsar/Gosper Gun", 20.0, help_y + 150.0, 18.0, WHITE);
            draw_text("Up/Down: Speed, H: Toggle help", 20.0, help_y + 170.0, 18.0, WHITE);
        }

        next_frame().await
    }
}
