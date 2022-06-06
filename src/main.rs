extern crate rand;

use rand::Rng;

fn main() {
    // generate 100 random number coordinates
    for _cnt in 0..100 {
        let x: f64 = rand::thread_rng().gen();
        let y: f64 = rand::thread_rng().gen();
        judge(x, y);
    }
}


fn judge(x: f64, y: f64) {
    println!("x: {} y: {}", x, y);
    // if (0.0 < x && x < 1.0) && (0.0 < y && y < 1.0) {
    //     println!("true");
    // }
}