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

    println!("The greatest common divisor of {:?} is {}", numbers, d);

    println!("some stuff");

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sv: &[f64] = &v;
    let _sa: &[f64] = &a;

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

    let mut s = "Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string(); // nothing is dropped here
    println!("{}, {}", t, s);

    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");
    // 2. Move a value out of a given index in the vector,
    // and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    // Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);

    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    #[derive(Copy, Clone)]
    struct Label {
        number: u32,
    }

    // #[derive(Copy, Clone)]
    struct StringLabel {
        name: String,
    }

    fn print_struct(l: Label) {
        println!("STAMP: {}", l.number);
    }

    let l = Label { number: 3 };
    print_struct(l);
    println!("My label number is: {}", l.number);

    use std::rc::Rc;
    // Rust can infer all these types; written out for clarity
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let _t: Rc<String> = s.clone();
    let _u: Rc<String> = s.clone();

    use std::collections::HashMap;
    type Table = HashMap<String, Vec<String>>;

    fn show(table: &Table) {
        for (artist, works) in table {
            println!("works by {}:", artist);
            for work in works {
                println!(" {}", work);
            }
        }
    }

    fn sort_works(table: &mut Table) {
        for (_artist, works) in table {
            works.sort();
        }
    }

    let mut table = Table::new();

    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );

    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );

    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    sort_works(&mut table);
    println!("----------");
    show(&table);

    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };

    let anime_ref = &aria;

    assert_eq!(anime_ref.name, "Aria: The Animation");
    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    assert_eq!(rrr.y, 729);

    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(rx == ry); // their referents are equal
    assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses

    static mut STASH: &i32 = &128;

    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }

    static WORTH_POINTING_AT: i32 = 1000;

    f(&WORTH_POINTING_AT);

    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }
    println!("{}", r);
}
