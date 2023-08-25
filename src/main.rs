use minifb::{Key, Window, WindowOptions};

mod rasterizer;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut window = Window::new(
        "Line drawing test",
        WIDTH,
        HEIGHT,
        WindowOptions { title: true, resize: true, ..Default::default() }
    )
    .expect("Unable to create window");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let (mut width, mut height) = (WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (w, h) = window.get_size();
        if w != width || h != height {
            if w * h > width * height {
                buffer.resize(w*h, 0);
            }
        }
        fill_pattern(&mut buffer[0..w*h], w, h, 10);
        window.update_with_buffer(&buffer[0..w*h], w, h);
        (width, height) = (w, h);
    }

}

fn fill_pattern(buffer: &mut [u32], width: usize, height: usize, gap: usize) {
    assert_eq!(buffer.len(), width * height);

    buffer
        .iter_mut()
        .enumerate()
        .for_each(|(i, p)| {
            if i % gap == 0 {
                *p = 0xffffffff;
            } else {
                *p = 0x0;
            }
        });
}
