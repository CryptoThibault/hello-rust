fn main() {
    let x = 42;
    if x < 42 {
        println!("plus petit que 42");
    } else if x == 42 {
        println!("42 tout rond");
    } else {
        println!("plus grand que 42");
    }
    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }
    println!("{}", x);
    let mut y = 0;
    while y != 42 {
        y += 1;
    }
    println!("{}", y);

    for x in 0..5 {
        println!("{}", x);
    }
    for x in 0..=5 {
        println!("{}", x);
    }

    let a = 120;
    match a {
        0 => { println!("Zero!"); }
        1 | 2 => { println!("Un ou deux!"); }
        3..=9 => { println!("Trois à huit!"); }
        num @ 10..=100 => { println!("{}!", num); }
        _ => { println!("Not found!"); }
    }

    let mut b = 0;
    let v = loop {
        b += 1;
        if b == 13 {
            break "J'ai trouvé le 13!";
        }
    };
    println!("La boucle dit: {}", v);
}
