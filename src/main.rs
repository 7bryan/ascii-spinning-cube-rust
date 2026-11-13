use std::io::{Write, stdout};

const WIDTH: usize = 80;
const HEIGHT: usize = 24;
const CUBE_WIDTH: i32 = 20;

fn rotate_3d(x: f64, y: f64, z: f64, a: f64, b: f64, c: f64) -> (f64, f64, f64) {
    // x axix rotation
    let xy: f64 = y * a.cos() - z * a.sin();
    let xz: f64 = y * a.sin() + z * a.cos();

    // y axis rotation
    let yx: f64 = x * b.cos() + xy * b.sin();
    let yz: f64 = -x * b.sin() + xz * b.cos();

    // z axis rotation
    let zx: f64 = yx * c.cos() - xy * c.sin();
    let zy: f64 = yx * c.sin() + xy * c.cos();

    (zx, zy, yz);
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

    if luminance > -0.2 {}

    let chars = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];
}

fn main() {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;

    print!("/x1b[2J");

    loop {
        let mut buffer = [' '; WIDTH * HEIGHT];
        let mut z_buffer = [0.0; WIDTH * HEIGHT];
    }
}
