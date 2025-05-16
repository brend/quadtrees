use macroquad::prelude::*;

const MAX_SIZE: usize = 4;

#[macroquad::main("quadtrees")]
async fn main() {
    let mut points = vec![];
    let point_count = 100;
    for _ in 0..point_count {
        points.push(Vec2::new(
            rand::gen_range(0.0, screen_width()),
            rand::gen_range(0.0, screen_height()),
        ));
    }
    let mut qt = Quadtree::new(0.0, 0.0, screen_width(), screen_height());
    let mut i = 0;
    let mut frame_count = 0;

    loop {
        clear_background(BLACK);
        for p in &points {
            let color = if qt.contains(p) { GREEN } else { WHITE };
            draw_circle(p.x, p.y, 2.0, color);
        }

        qt.draw();

        if frame_count % 30 == 0 && i < points.len() {
            qt.add(&points[i]);
            i += 1;
        }

        frame_count += 1;

        let count = points.iter().filter(|p| qt.contains(p)).count();
        draw_text(
            &format!(
                "points tucked away: {}%",
                100.0 * count as f32 / points.len() as f32
            ),
            20.0,
            20.0,
            20.0,
            WHITE,
        );

        let (mx, my) = mouse_position();
        if let Some(r) = qt.find(mx, my) {
            draw_rectangle_lines(r.x, r.y, r.w, r.h, 2.0, PINK);
        }

        next_frame().await;
    }
}

struct Quadtree {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    points: Vec<Vec2>,
    children: Vec<Box<Quadtree>>,
}

impl Quadtree {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Quadtree {
            x,
            y,
            w,
            h,
            points: vec![],
            children: vec![],
        }
    }

    fn add(&mut self, p: &Vec2) -> bool {
        // Check if point is within bounds (exclusive right and bottom edges)
        if p.x < self.x || p.x >= self.x + self.w || p.y < self.y || p.y >= self.y + self.h {
            return false;
        }

        // If this is a leaf node and has space, add the point
        if self.children.is_empty() && self.points.len() < MAX_SIZE {
            self.points.push(*p);
            return true;
        }

        // If this is a leaf node but full, subdivide and redistribute points
        if self.children.is_empty() {
            self.divide();
            // Redistribute existing points to children
            let mut points_to_redistribute = std::mem::take(&mut self.points);
            for point in points_to_redistribute {
                for child in self.children.iter_mut() {
                    if child.add(&point) {
                        break;
                    }
                }
            }
        }

        // Add the new point to the appropriate child
        for child in self.children.iter_mut() {
            if child.add(p) {
                return true;
            }
        }

        false
    }

    fn divide(&mut self) {
        if self.children.len() == 0 {
            let w_2 = self.w / 2.0;
            let h_2 = self.h / 2.0;
            self.children
                .push(Box::new(Quadtree::new(self.x, self.y, w_2, h_2)));
            self.children
                .push(Box::new(Quadtree::new(self.x + w_2, self.y, w_2, h_2)));
            self.children
                .push(Box::new(Quadtree::new(self.x, self.y + h_2, w_2, h_2)));
            self.children.push(Box::new(Quadtree::new(
                self.x + w_2,
                self.y + h_2,
                w_2,
                h_2,
            )));
        }
    }

    fn draw(&self) {
        draw_rectangle_lines(self.x, self.y, self.w, self.h, 1.0, GRAY);

        for child in &self.children {
            child.draw();
        }
    }

    fn contains(&self, p: &Vec2) -> bool {
        self.points.iter().any(|q| q == p) || self.children.iter().any(|c| c.contains(p))
    }

    fn find(&self, x: f32, y: f32) -> Option<Rect> {
        for c in &self.children {
            match c.find(x, y) {
                None => (),
                r => return r,
            }
        }
        if x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h {
            Some(Rect::new(self.x, self.y, self.w, self.h))
        } else {
            None
        }
    }
}
