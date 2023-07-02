use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;

const PLAYER_ROTATION_SPEED: f64 = 3.0;
const PLAYER_ACCELERATION: f64 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Right,
    None,
}

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed_x: f64,
    speed_y: f64,
    direction: Direction,
    angle: f64,
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);

    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());

    //The copy_ex() method is an even more powerful version of copy(). It has more parameters for extra things you might want to do when you copy the image (e.g. flip horizontal, flip vertical, rotate, etc.).
    canvas.copy_ex(texture, player.sprite, screen_rect, player.angle, None, false, false)?;

    canvas.present();

    Ok(())
}

fn update_player(player: &mut Player) {
    use self::Direction::*;
    match player.direction {
        Left => {
            player.angle -= PLAYER_ROTATION_SPEED;
            player.position = player.position.offset(player.speed_x as i32, -player.speed_y as i32);
        },
        Right => {
            player.angle += PLAYER_ROTATION_SPEED;
            player.position = player.position.offset(player.speed_x as i32, -player.speed_y as i32);
        },
        Up => {
            let impulse_x = player.angle.to_radians().sin() * PLAYER_ACCELERATION;
            let impulse_y = player.angle.to_radians().cos() * PLAYER_ACCELERATION;

            player.speed_x += impulse_x;
            player.speed_y += impulse_y;

            player.position = player.position.offset(player.speed_x as i32, -player.speed_y as i32);
        }
        None => {
            player.position = player.position.offset(player.speed_x as i32, -player.speed_y as i32);
        }
    }

  if player.position.y < - 300 - ((player.sprite.height() / 2) as i32) ||
    player.position.y > 300 + ((player.sprite.height() / 2) as i32) {
    player.position.y *= -1;
  }

  if player.position.x < - 400 - ((player.sprite.width() / 2) as i32) ||
    player.position.x > 400 + ((player.sprite.width() / 2) as i32) {
    player.position.x *= -1;
  }
}

fn main() -> Result<(), String> {
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "2");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to stay unused because if we don't have any variable at all then Rust will treat it as a temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("Spaceship", 800, 600)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("Could not make a canvas");


    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/spaceship.jpg")?;

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 100, 100),
        speed_x: 0.0,
        speed_y: 0.0,
        direction: Direction::None,
        angle: 0.0,
    };



    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.direction = Direction::Left;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.direction = Direction::Right;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.direction = Direction::Up;
                },
                Event::KeyUp { keycode: Some( Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some( Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some( Keycode::Up), repeat: false, .. } => {
                    player.direction = Direction::None;
                }
              _ => {}
            }
        }

        // Update
        update_player(&mut player);

        // Render
        render(
            &mut canvas,
            Color::RGB(255, 255, 255),
            &texture,
            &player,
        )?;

        // Timing
        let frame_rate = 30;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / frame_rate));
    }

    Ok(())
}
