struct Vector2 {
    x: f64,
    y: f64,
}

fn gradient(ix: f64, iy: f64) -> Vector2 {

    let xs: f64 = ix*21942.0+iy*171324.0+8912.0;
    let yc: f64 = ix*23157.0*iy*217832.0+6758.0;

    let rand: f64 = xs.sin() * yc.cos();

    let grad = Vector2 {
        x: rand.sin() as f64,
        y: rand.cos() as f64,
    };

    return grad
}

fn interpolate(a: f64, b: f64, w: f64) -> f64 {
    return w.powf(2.0) * (3.0-2.0*w) * (b - a) + a; 
}

fn dot_grid_gradient(ix: u64, iy: u64, x: f64, y: f64) -> f64 {
    let grad = gradient(ix as f64, iy as f64);

    let xdist = x - ix as f64;
    let ydist = y - iy as f64;

    return xdist*grad.x + ydist*grad.y;
}

pub fn perlin_noise(x: f64, y: f64) -> f64 {

    let flx = x as f64;
    let fly = y as f64;

    let fx = flx as u64;
    let fx1 = fx + 1;

    let fy = fly as u64;
    let fy1 = fy+1;

    let sx = flx - fx as f64;
    let sy = fly - fy as f64;

    return interpolate(interpolate(dot_grid_gradient(fx, fy, x, y),
                                   dot_grid_gradient(fx1,fy, x, y),
                                   sx), 
                                   interpolate(
                                   dot_grid_gradient(fx, fy1, x, y),
                                   dot_grid_gradient(fx1,fy1, x, y),
                                   sx), sy);
}