extern crate piston_window;
use piston_window::*;

const WINDOW_TITLE: &str = "UNKO";

const WINDOW_SIZE: Size = Size {
    width: 640.0,
    height: 480.0,
};

const FPS: u64 = 60;

struct Haniwa {
    texture: G2dTexture,
    size: Size,
    scale: f64,
    pos: Position,
}

impl Haniwa {
    fn draw(&self, c: Context, g: &mut G2d) {
        let transform = c
            .transform
            .trans(
                self.pos.x as f64 - self.screen_size().width / 2.0,
                self.pos.y as f64 - self.screen_size().width / 2.0,
            )
            .scale(self.scale, self.scale);
        image(&self.texture, transform, g);
    }

    fn sled(&mut self, key: &ArrowKeysState) {
        if key.up {
            if self.pos.y > (self.screen_size().height / 2.0) as i32 {
                self.pos.y -= Haniwa::SPEED;
            } else {
                self.pos.y = (self.screen_size().height / 2.0) as i32;
            }
        }
        if key.down {
            if self.pos.y < (WINDOW_SIZE.height - self.screen_size().height / 2.0) as i32 {
                self.pos.y += Haniwa::SPEED;
            } else {
                self.pos.y = (WINDOW_SIZE.height - self.screen_size().height / 2.0) as i32;
            }
        }
        if key.left {
            if self.pos.x > (self.screen_size().width / 2.0) as i32 {
                self.pos.x -= Haniwa::SPEED;
            } else {
                self.pos.x = (self.screen_size().width / 2.0) as i32;
            }
        }
        if key.right {
            if self.pos.x < (WINDOW_SIZE.width - self.screen_size().width / 2.0) as i32 {
                self.pos.x += Haniwa::SPEED;
            } else {
                self.pos.x = (WINDOW_SIZE.width - self.screen_size().width / 2.0) as i32;
            }
        }
    }

    const SPEED: i32 = 2;

    fn screen_size(&self) -> Size {
        return Size {
            width: self.size.width * self.scale,
            height: self.size.height * self.scale,
        };
    }
}

struct ArrowKeysState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
impl ArrowKeysState {
    fn new() -> ArrowKeysState {
        return ArrowKeysState {
            up: false,
            down: false,
            left: false,
            right: false,
        };
    }
    ///`PistonWindow::ButtonArgs`から状態をセットする
    fn set(&mut self, key: &ButtonArgs) {
        match key.button {
            Button::Keyboard(Key::Up) => {
                self.up = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::Down) => {
                self.down = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::Left) => {
                self.left = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            Button::Keyboard(Key::Right) => {
                self.right = if key.state == ButtonState::Press {
                    true
                } else {
                    false
                };
            }
            _ => {}
        }
    }
}

fn main() {
    println!("Hello, world!!");
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(true)
        .samples(4)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.events.set_max_fps(FPS);
    window.events.set_ups(FPS);

    let mut haniwa = Haniwa {
        texture: Texture::from_path(
            &mut window.create_texture_context(),
            "./asset/img/haniwa.png",
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap(),
        size: Size {
            width: 800.0,
            height: 800.0,
        },
        scale: 0.2,
        pos: Position {
            x: (WINDOW_SIZE.width / 2.0) as i32,
            y: (WINDOW_SIZE.height / 2.0) as i32,
        },
    };

    let mut arrow_keys = ArrowKeysState::new();

    //メインループ
    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(_)) => {
                //レンダリング
                window.draw_2d(&e, |c, g, _| {
                    //画面を黒でクリア
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    //サンタさんを描画
                    haniwa.draw(c, g);
                });
            }
            Event::Loop(Loop::Update(_)) => {
                //アップデート
                haniwa.sled(&arrow_keys);
            }
            Event::Input(i, _) => {
                //入力関係
                if let Input::Button(key) = i {
                    arrow_keys.set(&key);
                }
            }
            _ => {}
        }
    }
}
