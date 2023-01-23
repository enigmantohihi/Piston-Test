extern crate piston_window;
use piston_window::{*, color::{WHITE, BLACK}};

const WINDOW_TITLE: &str = "OTO GAME";

const WINDOW_SIZE: Size = Size {
    width: 640.0,
    height: 480.0,
};

const END_POS: [Position;2] = [
    Position {x:-200,y:50},
    Position {x:-200,y:-50},
];

const FPS: u64 = 60;

struct Notes {
    pos: Position,
    radius: f64,
    time: f64,
    speed: f64,
    lane: u64,
    dir: u64,
    alive: bool,
    active: bool,
    finish: bool,
}

impl Notes {
    fn draw(&self, c: Context, g: &mut G2d) {
        let transform = c
            .transform
            .trans(
                self.screen_size().width / 2.0 + self.pos.x as f64,
                self.screen_size().height / 2.0 + self.pos.y as f64,
            );
        let rect = [0.0,0.0,self.radius,self.radius];
        ellipse(BLACK, rect, transform, g);
    }

    fn screen_size(&self) -> Size{
        return Size {
            width: WINDOW_SIZE.width,
            height: WINDOW_SIZE.height,
        };
    }
}

fn draw_shapes(c: Context, g: &mut G2d, line_number: u32) {
    let i = if line_number==1 {0} else {1};
    let thick = 5.0;
    let line_rect = [0.0,thick,0.0,0.0];
    let line_pos = Position {
        x: 0,
        y: END_POS[i].y - (thick/2.0)as i32,
    };
    line(BLACK, WINDOW_SIZE.width, line_rect, c.transform
        .trans(
            WINDOW_SIZE.width / 2.0 + line_pos.x as f64,
            WINDOW_SIZE.height / 2.0 + line_pos.y as f64,
        ), g);

    let radius = 40;
    let radius2 = 32;
    let circle_rect = [0.0,0.0,radius as f64,radius as f64];
    let circle_rect2 = [0.0,0.0,radius2 as f64,radius2 as f64];
    let circle_pos = Position {
        x: END_POS[i].x - (radius/2.0 as i32),
        y: END_POS[i].y - (radius/2.0 as i32),
    };
    let circle_pos2 = Position {
        x: END_POS[i].x - (radius2/2.0 as i32),
        y: END_POS[i].y - (radius2/2.0 as i32),
    };
    ellipse(BLACK ,circle_rect, c.transform
        .trans(
            WINDOW_SIZE.width / 2.0 + circle_pos.x as f64,
            WINDOW_SIZE.height / 2.0 + circle_pos.y as f64,
        ), g);
    ellipse(WHITE ,circle_rect2, c.transform
        .trans(
            WINDOW_SIZE.width / 2.0 + circle_pos2.x as f64,
            WINDOW_SIZE.height / 2.0 + circle_pos2.y as f64,
        ), g);
}

pub fn main() {
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE,WINDOW_SIZE)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .samples(4)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.events.set_max_fps(FPS);
    window.events.set_ups(FPS);

    let mut notes = Notes {
        pos: Position { x: (0) as i32, y: (0) as i32},
        radius: 40.0,
        time: 1.0,
        speed: 1.0,
        lane: 1,
        dir: 1,
        alive: true,
        active: true,
        finish: false,
    };

    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&e, |c, g, _| {
                    clear(WHITE, g);
                    notes.draw(c, g);
                    // rectangle(BLACK, square, transform, g)
                    draw_shapes(c, g, 1);
                    draw_shapes(c, g, 2);
                });
            }

            Event::Loop(Loop::Update(_)) => {
                notes.pos.x += 1;
            }

            _ => {}
        }
    }
}