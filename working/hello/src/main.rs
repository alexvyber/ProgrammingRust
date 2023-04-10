// use std::env;
// use std::str::FromStr;

// fn gcd(mut n: u64, mut m: u64) -> u64 {
//     assert!(n != 0 && m != 0);
//     while m != 0 {
//         if m < n {
//             let t = m;
//             m = n;
//             n = t;
//         }
//         m = m % n;
//     }
//     n
// }

fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}

fn main() {
    // let mut numbers = Vec::new();

    // for arg in env::args().skip(1) {
    //     numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    // }

    // if numbers.len() == 0 {
    //     eprintln!("Usage: gcd NUMBER ...");
    //     std::process::exit(1);
    // }

    // let mut d = numbers[0];

    // for m in &numbers[1..] {
    //     d = gcd(d, *m);
    // }

    // println!("The greatest common divisor of {:?} is {}", numbers, d);

    println!("some stuff");

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print(&v[0..2]); // print the first two elements of v
    print(&a[2..]); // print elements of a starting with a[2]
    print(&sv[1..3]); // print v[1] and v[2]

    println!(
        r####"
This raw string started with 'r###"'.
Therefore it does not end until we reach a quote mark ('"')
followed immediately by three pound signs ('###'):
###""####
    );

    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
}
