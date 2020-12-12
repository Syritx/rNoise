mod noise;

fn main() {

    let resolution = 56.0;
    let frequency = 10.0;
    let amplitude = 2.0;

    for x in 0..resolution as u64 {
        for y in 0..resolution as u64 {

            let nx = x as f64 / resolution * frequency;
            let ny = y as f64 / resolution * frequency;

            let a: f64 = noise::perlin_noise(nx, ny)*amplitude;
            println!("{}, x:{}, y:{}", a, nx, ny)
        }
    }
}