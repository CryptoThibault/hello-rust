struct Bag<T> {
    item: Option<T>,
}
struct SimpleBag<T> {
    item: T,
}

fn do_something_that_might_fail(i:i32) -> Result<f32,String> {
    if i == 13 {
        Ok(13.0)
    } else {
        Err(String::from("ce n'est pas le bon nombre"))   
    }
}

fn main() -> Result<(), String> {
    let i32_bag = Bag::<i32> { item: None };
    if i32_bag.item.is_none() {
        println!("il n'y a rien dans le sac!");
    } else {
        println!("il y a quelque chose dans le sac!");
    }
    let i32_bag = Bag::<i32> { item: Some(42) };
    if i32_bag.item.is_some() {
        println!("il y a quelque chose dans le sac!");
    } else {
        println!("il n'y a rien dans le sac!");
    }
    match i32_bag.item {
        Some(v) => println!("il y {} dans le sac!", v),
        None => println!("le sac est vide"),
    }

    let bool_bag = SimpleBag::<bool> { item: true };
    let float_bag = SimpleBag {item: 3.14 };
    let bag_in_bag = SimpleBag {
        item: SimpleBag { item: "bagception"},
    };
    println!("{}, {}, {}", 
        bool_bag.item,
        float_bag.item,
        bag_in_bag.item.item
    );

    let result = do_something_that_might_fail(13);

    match result {
        Ok(v) => println!("trouvé {}", v),
        Err(e) => {
            println!("Erreur: {}", e);
            return Err(String::from("quelque chose s'est mal passé dans le main!"));
        }
    }
    let v = do_something_that_might_fail(13)?;
    println!("trouvé {}", v);
    let w = do_something_that_might_fail(13).unwrap();
    println!("trouvé {}", w);

    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);
    println!("{}, {}, {}", i32_vec[0], i32_vec[1], i32_vec[2]);
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);
    println!("{}, {}, {}", float_vec[0], float_vec[1], float_vec[2]);
    let string_vec = vec![String::from("Hello"), String::from("World")];
    for word in string_vec {
        println!("{}", word);
    }

    Ok(())
}
