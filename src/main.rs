use macroquad::prelude::*;

const COLOR_BG: Color = Color::new(0.11, 0.11, 0.16, 1.0);
const COLOR_PANEL: Color = Color::new(0.18, 0.18, 0.24, 0.95);
const COLOR_BTN: Color = Color::new(0.25, 0.25, 0.32, 1.0);
const COLOR_BTN_HOVER: Color = Color::new(0.35, 0.35, 0.45, 1.0);
const COLOR_BTN_ACTIVE: Color = Color::new(0.53, 0.70, 0.98, 1.0);
const COLOR_TEXT: Color = WHITE;
const COLOR_ACCENT: Color = Color::new(0.98, 0.70, 0.53, 1.0);

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

    fn place_glider(&mut self, x: usize, y: usize) {
        let pattern = [(1,0),(2,1),(0,2),(1,2),(2,2)];
        for (dx, dy) in pattern { self.set_tile(x+dx, y+dy, 1); }
    }

    fn place_pulsar(&mut self, x: usize, y: usize) {
        let pattern = [
            (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
            (0,2),(5,2),(7,2),(12,2), (0,3),(5,3),(7,3),(12,3),
            (0,4),(5,4),(7,4),(12,4), (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),
            (2,7),(3,7),(4,7),(8,7),(9,7),(10,7), (0,8),(5,8),(7,8),(12,8),
            (0,9),(5,9),(7,9),(12,9), (0,10),(5,10),(7,10),(12,10),
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

struct Camera {
    x: f32, y: f32, zoom: f32,
}

impl Camera {
    fn new() -> Self { Camera { x: 0.0, y: 0.0, zoom: 1.0 } }
    fn screen_to_world(&self, sx: f32, sy: f32) -> (f32, f32) {
        ((sx - self.x) / self.zoom, (sy - self.y) / self.zoom)
    }
}

fn ui_button(x: f32, y: f32, w: f32, h: f32, text: &str, is_active: bool) -> bool {
    let (mx, my) = mouse_position();
    let is_hovered = mx >= x && mx <= x + w && my >= y && my <= y + h;
    
    let bg = if is_active {
        COLOR_BTN_ACTIVE
    } else if is_hovered {
        COLOR_BTN_HOVER
    } else {
        COLOR_BTN
    };

    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 1.0, Color::new(0.0, 0.0, 0.0, 0.3));

    let text_dim = measure_text(text, None, 18, 1.0);
    let tx = x + (w - text_dim.width) / 2.0;
    let ty = y + h / 2.0 + text_dim.height / 3.0;
    
    draw_text(text, tx, ty, 18.0, COLOR_TEXT);

    is_hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn ui_speed_button(x: f32, y: f32, w: f32, h: f32, text: &str) -> bool {
    ui_button(x, y, w, h, text, false)
}

#[macroquad::main("Game of Life")]
async fn main() {
    let rows = 100;
    let cols = 150;
    let mut map = Map::generate(rows, cols, BoundaryType::Toroidal);
    map.place_gosper_gun(2, 2);

    let mut camera = Camera::new();
    let cell_size: f32 = 8.0;
    let mut paused = true;
    let mut speed: f32 = 10.0;
    let mut accumulator: f32 = 0.0;
    let panel_w = 240.0;

    loop {
        let dt = get_frame_time();
        let (mx, my) = mouse_position();
        
        let window_w = screen_width();
        let window_h = screen_height();
        let panel_h = window_h;
        let panel_x = window_w - panel_w;
        
        let is_mouse_over_ui = mx >= panel_x;

        if !is_mouse_over_ui {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (wx, wy) = camera.screen_to_world(mx, my);
                let cx = (wx / cell_size) as i32;
                let cy = (wy / cell_size) as i32;
                if cx >= 0 && cy >= 0 {
                    map.toggle_tile(cx as usize, cy as usize);
                }
            }

            if is_mouse_button_down(MouseButton::Middle) || is_mouse_button_down(MouseButton::Right) {
                let delta = mouse_delta_position();
                camera.x += delta.x;
                camera.y += delta.y;
            }

            let (_, wheel_y) = mouse_wheel();
            if wheel_y != 0.0 {
                let factor = if wheel_y > 0.0 { 1.1 } else { 1.0 / 1.1 };
                let wx = (mx - camera.x) / camera.zoom;
                let wy = (my - camera.y) / camera.zoom;
                camera.zoom *= factor;
                camera.x = mx - wx * camera.zoom;
                camera.y = my - wy * camera.zoom;
            }
        }

        if !paused {
            accumulator += dt * speed;
            while accumulator >= 1.0 {
                map.update();
                accumulator -= 1.0;
            }
        }

        clear_background(COLOR_BG);

        for y in 0..map.grid.len() {
            for x in 0..map.grid[0].len() {
                let sx = camera.x + x as f32 * cell_size * camera.zoom;
                let sy = camera.y + y as f32 * cell_size * camera.zoom;
                let size = cell_size * camera.zoom - 1.0;

                if sx + size < 0.0 || sy + size < 0.0 || sx > window_w || sy > window_h {
                    continue;
                }

                let color = if map.grid[y][x] == 1 {
                    Color::new(0.6, 0.95, 0.75, 1.0)
                } else {
                    Color::new(0.15, 0.15, 0.2, 1.0)
                };
                draw_rectangle(sx, sy, size, size, color);
            }
        }

        draw_rectangle(panel_x, 0.0, panel_w, panel_h, COLOR_PANEL);
        draw_line(panel_x, 0.0, panel_x, panel_h, 2.0, Color::new(0.0, 0.0, 0.0, 0.5));

        let mut ui_y = 20.0;
        let margin = 15.0;
        let btn_w = panel_w - margin * 2.0;
        let btn_h = 36.0;

        draw_text("GAME OF LIFE", panel_x + margin, ui_y + 20.0, 22.0, COLOR_ACCENT);
        ui_y += 50.0;

        draw_text(&format!("Gen: {}", map.generation), panel_x + margin, ui_y + 15.0, 16.0, COLOR_TEXT);
        draw_text(&format!("Alive: {}", map.alive_count()), panel_x + margin + 120.0, ui_y + 15.0, 16.0, COLOR_TEXT);
        ui_y += 30.0;

        draw_text("PLAYBACK", panel_x + margin, ui_y + 15.0, 14.0, Color::new(0.6, 0.6, 0.7, 1.0));
        ui_y += 25.0;

        if ui_button(panel_x + margin, ui_y, btn_w, btn_h, if paused { "▶  RESUME" } else { "⏸  PAUSE" }, paused) {
            paused = !paused;
        }
        ui_y += btn_h + 10.0;

        draw_text(&format!("Speed: {:.1}x", speed), panel_x + margin + 60.0, ui_y + btn_h / 2.0 + 6.0, 18.0, COLOR_TEXT);
        if ui_speed_button(panel_x + margin, ui_y, 50.0, btn_h, " - ") {
            speed = (speed / 1.2).max(1.0);
        }
        if ui_speed_button(panel_x + margin + btn_w - 50.0, ui_y, 50.0, btn_h, " + ") {
            speed = (speed * 1.2).min(60.0);
        }
        ui_y += btn_h + 10.0;

        if ui_button(panel_x + margin, ui_y, btn_w, btn_h, "⏭  STEP (N)", false) {
            map.update();
        }
        ui_y += btn_h + 25.0;

        draw_text("BOARD ACTIONS", panel_x + margin, ui_y + 15.0, 14.0, Color::new(0.6, 0.6, 0.7, 1.0));
        ui_y += 25.0;

        let boundary_text = match map.boundary {
            BoundaryType::Bounded => "Bounded (Edges)",
            BoundaryType::Toroidal => "Toroidal (Wrap)",
        };
        if ui_button(panel_x + margin, ui_y, btn_w, btn_h, boundary_text, false) {
            map.boundary = match map.boundary {
                BoundaryType::Bounded => BoundaryType::Toroidal,
                BoundaryType::Toroidal => BoundaryType::Bounded,
            };
        }
        ui_y += btn_h + 10.0;

        if ui_button(panel_x + margin, ui_y, btn_w / 2.0 - 5.0, btn_h, "🎲 Random", false) {
            map.fill_random();
        }
        if ui_button(panel_x + margin + btn_w / 2.0 + 5.0, ui_y, btn_w / 2.0 - 5.0, btn_h, "🗑 Clear", false) {
            map.clear();
        }
        ui_y += btn_h + 25.0;

        draw_text("PATTERNS", panel_x + margin, ui_y + 15.0, 14.0, Color::new(0.6, 0.6, 0.7, 1.0));
        ui_y += 25.0;

        if ui_button(panel_x + margin, ui_y, btn_w, btn_h, "Glider", false) {
            map.clear();
            map.place_glider(5, 5);
        }
        ui_y += btn_h + 10.0;

        if ui_button(panel_x + margin, ui_y, btn_w, btn_h, "Pulsar", false) {
            map.clear();
            map.place_pulsar(10, 10);
        }
        ui_y += btn_h + 10.0;

        if ui_button(panel_x + margin, ui_y, btn_w, btn_h, "Gosper Glider Gun", false) {
            map.clear();
            map.place_gosper_gun(2, 2);
        }
        ui_y += btn_h + 25.0;

        draw_text("LMB: Draw cells", panel_x + margin, ui_y + 15.0, 14.0, Color::new(0.5, 0.5, 0.6, 1.0));
        ui_y += 20.0;
        draw_text("RMB/MMB: Pan", panel_x + margin, ui_y + 15.0, 14.0, Color::new(0.5, 0.5, 0.6, 1.0));
        ui_y += 20.0;
        draw_text("Scroll: Zoom", panel_x + margin, ui_y + 15.0, 14.0, Color::new(0.5, 0.5, 0.6, 1.0));

        next_frame().await
    }
}
