//2.1 if/else
// fn main(){ 
//     let x = 5; 
//     if x < 5 { 
//         println!("x is less than 5");
//     } else if x > 5 
//     { println!("x is greater than 5");} else{ 
//         println!("x is equal to 5");
//     }
// }

//2.2 loop (infinite)
// fn main(){ 
//     let mut x =0;
//     loop {
//         x += 1;
//         if x == 42 {
//             break;
//         }
//     }
//     println!("x:{}",x)
// }

// 2.3 while
// fn main(){ 
//     let mut x = 0; 
//     while x < 42{ 
//         x += 1;
//     }
//
//     println!("x:{}",x);
// }

//2.4 for loops
// fn main() {
//     for x in 0..5 { 
//         println!("{}",x)
//     }
//
//     for x in 0..=5 { 
//         println!("{}",x)
//     }
// }

//2.5  match statements
// fn main(){ 
//     let x = 11;
//     // match must contain all the possible  expressions
//     match x {
//         0..10 =>{ 
//             println!("x lies between 0-9")
//         }
//         10 => { 
//             println!("x = 10")
//         } 
//
//         matched_num @ 11..=100 => { 
//             println!("x lies between 10-100. matched_num:{}", matched_num)
//         }
//         _ => {} //wild card
//     }
// }

// 2.6 Returning the values from the loops
// fn main(){ 
//     let mut x = 0; 
//     let v = loop { 
//         x += 1;
//         if x == 10 { break "Hello world"} //this break value gets stored in v
//     };
//
//     println!("v:{:?}", v);
// }

// 2.7 Returning values from block expression
fn main(){ 
    println!("value from function:{}", example());
}

fn example() -> i32 { 
    let x = 43;
    let v = if x < 42 { -1 } else { 1 };
    println!("v:{}",v);

    let food = "hamburger";
    let result = match food { 
        "hamburger" => "food is hamburger",
        _ => "no food",
    };
    println!("Identity of food:{}", result);

    let v = {
        //this scope block result without polluting function scope
        let a = 1; 
        let b = 2; 
        
        a+b
    };

    v+4
}
