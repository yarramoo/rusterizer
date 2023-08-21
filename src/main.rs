use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut window = Window::new(
        "Line drawing test",
        WIDTH,
        HEIGHT,
        WindowOptions {..WindowOptions::default()},
    )
    .expect("Unable to create window");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if (row + col) % 5 == 0 {
                buffer[col + WIDTH*row] = 0xffffffff;
            }
        }
    }


    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT);
    }

}
