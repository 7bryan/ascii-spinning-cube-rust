use std::io::{Write, stdout};
use std::thread::sleep;
use std::time::Duration;

const WIDTH: usize = 80;
const HEIGHT: usize = 38;
const CUBE_WIDTH: f64 = 20.0;
const INCREMENT_SPEED: f64 = 0.6;

fn rotate_3d(x: f64, y: f64, z: f64, a: f64, b: f64, c: f64) -> (f64, f64, f64) {
    // x axix rotation
    let xy: f64 = y * a.cos() - z * a.sin();
    let xz: f64 = y * a.sin() + z * a.cos();

    // y axis rotation
    let yx: f64 = x * b.cos() + xz * b.sin();
    let yz: f64 = -x * b.sin() + xz * b.cos();

    // z axis rotation
    let zx: f64 = yx * c.cos() - xy * c.sin();
    let zy: f64 = yx * c.sin() + xy * c.cos();

    (zx, zy, yz)
}

fn calculate_surface(
    cube_x: f64,
    cube_y: f64,
    cube_z: f64,
    norm_x: f64,
    norm_y: f64,
    norm_z: f64,
    a: f64,
    b: f64,
    c: f64,
    buffer: &mut [char],
    z_buffer: &mut [f64],
) {
    let (x, y, z) = rotate_3d(cube_x, cube_y, cube_z, a, b, c);
    let (nx, ny, nz) = rotate_3d(norm_x, norm_y, norm_z, a, b, c);

    let light_x = -0.577;
    let light_y = 0.577;
    let light_z = -0.577;

    let luminance = (nx * light_x) + (ny * light_y) + (nz * light_z);

    if luminance > -0.2 {
        let mut lum_index = (luminance * 11.0).round() as isize;
        if lum_index < 0 {
            lum_index = 0;
        }
        if lum_index > 11 {
            lum_index = 11;
        }

        let chars = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];
        let ch = chars[lum_index as usize];

        let z_offset = 100.0;
        let projected_z = z + z_offset;
        let ooz = 1.0 / projected_z;

        let k1 = 40.0;

        let xp = (WIDTH as f64 / 2.0 + k1 * ooz * x * 2.0) as isize;
        let yp = (HEIGHT as f64 / 2.0 + k1 * ooz * y) as isize;

        if xp >= 0 && xp < WIDTH as isize && yp >= 0 && yp < HEIGHT as isize {
            let idx = (xp + yp * WIDTH as isize) as usize;
            if ooz > z_buffer[idx] {
                z_buffer[idx] = ooz;
                buffer[idx] = ch;
            }
        }
    }
}

fn main() {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;

    print!("\x1b[2J");

    loop {
        let mut buffer = [' '; WIDTH * HEIGHT];
        let mut z_buffer = [0.0; WIDTH * HEIGHT];

        let mut cube_x = -CUBE_WIDTH;
        while cube_x < CUBE_WIDTH {
            let mut cube_y = -CUBE_WIDTH;
            while cube_y < CUBE_WIDTH {
                // front back
                calculate_surface(
                    cube_x,
                    cube_y,
                    CUBE_WIDTH,
                    0.0,
                    0.0,
                    1.0,
                    a,
                    b,
                    c,
                    &mut buffer,
                    &mut z_buffer,
                );
                calculate_surface(
                    cube_x,
                    cube_y,
                    -CUBE_WIDTH,
                    0.0,
                    0.0,
                    -1.0,
                    a,
                    b,
                    c,
                    &mut buffer,
                    &mut z_buffer,
                );

                // top bot
                calculate_surface(
                    cube_x,
                    CUBE_WIDTH,
                    cube_y,
                    0.0,
                    1.0,
                    0.0,
                    a,
                    b,
                    c,
                    &mut buffer,
                    &mut z_buffer,
                );
                calculate_surface(
                    cube_x,
                    -CUBE_WIDTH,
                    cube_y,
                    0.0,
                    -1.0,
                    0.0,
                    a,
                    b,
                    c,
                    &mut buffer,
                    &mut z_buffer,
                );

                // left right
                calculate_surface(
                    CUBE_WIDTH,
                    cube_y,
                    cube_x,
                    1.0,
                    0.0,
                    0.0,
                    a,
                    b,
                    c,
                    &mut buffer,
                    &mut z_buffer,
                );
                calculate_surface(
                    -CUBE_WIDTH,
                    cube_y,
                    cube_x,
                    -1.0,
                    0.0,
                    0.0,
                    a,
                    b,
                    c,
                    &mut buffer,
                    &mut z_buffer,
                );

                cube_y += INCREMENT_SPEED;
            }
            cube_x += INCREMENT_SPEED;
        }

        print!("\x1b[H");

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", buffer[y * WIDTH + x]);
            }
            println!();
        }

        stdout().flush().unwrap();

        a += 0.05;
        b += 0.05;
        c += 0.01;

        sleep(Duration::from_millis(16));
    }
}
