// 3.1 Structs
// struct Car{ 
//     wheel: i8,
//     brand: String, 
//     color: String, 
//     electric: bool,
// }

// 3.2 calling methodes
// fn main(){ 
//     // static method  to create instance of string
//     let s = String::from("Hello world");
//
//     // using a method on instance
//     println!("length of string:{}", s.len())
// }

//3.3 creating data in memory
// fn main(){ 
//     let test_case= &Car{ 
//         wheel: 2, 
//         brand: String::from("lamborgini"),
//         color: String::from("blue"),
//         electric: false,
//     };
//     println!(
//         "car: {},{}, {}, {}",
//         test_case.wheel, test_case.brand, test_case.color, test_case.electric );
// }

//3.4 tuple like struct
// struct Location(u8, u8);
// fn main(){
//     let loc = Location(42, 32);
//     println!("{}, {}", loc.0, loc.1);
// }

//3.5 unit like struct
//unit -> empty tuple
// struct Helo; //empty struct
// fn main(){ 
//     let _hello = Helo;
// }

//3.6 Enums and Enums with data
#![allow(dead_code)]

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam
}
enum Size { Big, Small}
enum Weapon{
    Claw (i32, Size),
    None
}

struct SeaCreature {
    species: Species,
    weapon: Weapon, 
    name: String,
}

fn main() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        weapon: Weapon::Claw(2, Size::Big)
    };

    match ferris.species {
        Species::Crab =>{
            match ferris.weapon {
                Weapon::Claw(num_claw, size) =>{
                    let size_desc = match size{ 
                       Size::Big => "big",
                       Size::Small => "small",
                    };
                println!("name:{}, claws:{}, size:{}",ferris.name,num_claw ,size_desc);
                },
                _ => {println!("{} is a crab with no weapon",ferris.name); },
            }
        },
        Species::Octopus => println!("{} is a octopus",ferris.name),
        Species::Fish => println!("{} is a fish",ferris.name),
        _ => print!("{} is some other animal", ferris.name)
    }
}
