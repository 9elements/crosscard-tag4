use anyhow::Result;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

fn main() -> Result<()> {
    let mut window = Window::new(
        "Robo-Rage",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X16,
            ..Default::default()
        },
    )?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        // TODO: Draw the robots

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }

    Ok(())
}
