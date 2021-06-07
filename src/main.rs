use ferris_says::say;
use std::io::{stdout, BufWriter};

struct Location(i32, i32);
struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}


fn main() {
    let ferris = SeaCreature {
        animal_type: String::from("crabe"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("pinces"),
    };
    let location = Location(25, -43);
    println!("{} est un {}. Ces derniers possèdent {} bras, {} jambes, et utilisent leurs {} comme arme.", 
    ferris.name,
    ferris.animal_type,
    ferris.arms,
    ferris.legs,
    ferris.weapon);
    println!("Je me trouve à  la position x: {}, y: {}", location.0, location.1);

    let stdout = stdout();
    let message = String::from(ferris.name);
    let width = message.len();

    println!("{} contient {} caractéres.", message, width);

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

}
