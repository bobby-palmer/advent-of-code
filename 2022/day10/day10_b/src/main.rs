const WIDTH: usize = 40;
const HEIGHT: usize = 6;

fn main() {
    let input = include_str!("../../input.in");
    let mut screen = Screen::new();
    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| match line {
            "noop" => screen.noop(),
            _ => {
                let (_, amount) = line.split_at(5);
                let x = amount.parse().unwrap();
                screen.addx(x);
            }
        });
    println!("here is your message :");
    println!("{screen}");
}
struct Screen {
    sprite: i32,
    pixel: (usize, usize),
    screen: [[char; WIDTH]; HEIGHT],
    cycle: usize,
}
impl Screen {
    fn new() -> Self {
        Screen {
            sprite: 1,
            pixel: (0, 0),
            screen: [['.'; WIDTH]; HEIGHT],
            cycle: 0,
        }
    }
    fn pixel_in_sprite(&self) -> bool {
        let current = self.pixel.0 as i32;
        (current == self.sprite) || (current == (self.sprite - 1)) || (current == (self.sprite + 1))
    }
    fn noop(&mut self) {
        self.render_pxl();
        self.cycle();
    }
    fn addx(&mut self, x: i32) {
        self.render_pxl();
        self.cycle();
        self.render_pxl();
        self.cycle();
        let new = self.sprite as i32 + x;
        // this will panic if the `sprite` variable goes negative
        self.sprite = new;
    }
    fn cycle(&mut self) {
        self.increment_pixel();
        self.cycle += 1;
    }
    fn increment_pixel(&mut self) {
        self.pixel.0 += 1;
        // if past edge move to next line
        if self.pixel.0 == WIDTH {
            self.pixel.0 = 0;
            self.pixel.1 += 1;
        }
    }
    fn draw_pix(&mut self) {
        let (x, y) = self.pixel;
        self.screen[y][x] = '#';
    }
    fn render_pxl(&mut self) {
        if self.pixel_in_sprite() {
            self.draw_pix();
        }
    }
}
impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.screen.iter() {
            for pixel in row.iter() {
                write!(f, "{pixel}")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}
