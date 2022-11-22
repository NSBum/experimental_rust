use rand::prelude::*;

fn maybe() -> Option<f64> {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    if y < 0.5 {
        None
    } else {
        // this explicit else {} block is required
        Some(y)
    }
}

fn main() {
    match maybe() {
        None => println!("Nothing"),
        Some(float) => {
            println!("value {}", float);
        }
    }
}
