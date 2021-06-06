const PI: f32 = 3.14159;

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}
fn make_nothing() -> () {
    return ();
}

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5;
    println!("Number {}", x);

    println!(
        "π est le rapport de la circonférence d’un cercle à
        son diamètre et sa valeur approximative est {}",
        PI
    );

    let nums: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", nums);
    println!("{}", nums[1]);

    println!("{}", add(15, -54));

    let result = swap(46, 64); 
    println!("{} {}", result.0, result.1);
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);

    let nothing = make_nothing();
    println!("Nothing : {:?}", nothing);
}
