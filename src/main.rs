extern crate rand;

use rand::Rng;

fn main() {
    let range: i32 = 10000000;
    let mut cnt: i32 = 0;
    // generate 100 random number coordinates
    for _cnt in 0..range {
        let x: f64 = rand::thread_rng().gen();
        let y: f64 = rand::thread_rng().gen();
        let judgeresult: bool = judge(x, y);
        if judgeresult == true {
            cnt += 1;
        }
    }
    println!("cnt {}", cnt);
}


fn judge(x: f64, y: f64) -> bool {
    let xplusy = x.powf(2.0) + y.powf(2.0);
    let distance = xplusy.sqrt();
    if distance >= 1.0 {
        return false;
    }
    return true;
}