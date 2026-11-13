use std::io::{Write, stdout};

const WIDTH: i32 = 80;
const HEIGHT: i32 = 24;
const CUBE_WIDTH: i32 = 20;

fn rotate_3d(x: f64, y: f64, z: f64, a: f64, b: f64, c: f64) -> () {
    // x axix rotation
    let xy = y * a.cos() - z * a.sin();
    let xz = y * a.sin() + z * a.cos();

    // y axis rotation
    let yx = x * b.cos() + xy * b.sin();
    let yz = -x * b.sin() + xz * b.cos();

    // z axis rotation
    let zx = yx * c.cos() - xy * c.sin();
    let zy = yx * c.sin() + xy * c.cos();

    (zx, zy, yz);
}

fn main() {
    println!("test");
}
