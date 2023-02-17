extern crate geosym_wasm;

use geosym_wasm::compute_fft;

fn main() {
    // println!("Hello, world!");
    let v = vec![0.0; 12];
    println!("{:?}",compute_fft(12,v));
}