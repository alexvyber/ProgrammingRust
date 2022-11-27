use num::Complex;
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

struct Complex<T> {
    /// Real portion of the complex number
    re: T,
    /// Imaginary portion of the complex number
    im: T,
}

enum Option<T> {
    None,
    Some(T),
    }
    
/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius 2 centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
    if z.norm_sqr() > 4.0 {
    return Some(i);
    }
    z = z * z + c;
    }
    None
    }

fn main() {
    println!("Hello, world!");
}
