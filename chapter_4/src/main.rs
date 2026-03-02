// // ownership
// fn main(){ 
//     let s = String::from("Hello world");
//     takes_ownership(s);             //value of s is moved into the function
//
//     // println!("{s}");
//     // this would throw ownership error, so this is invalid
//
//     let x = 5;
//     makes_copt(x);
//     println!("{x}");
// }
//
// fn takes_ownership(hw:String){ 
//     println!("{hw}")
// }
//
// fn makes_copt(some_int:i32){ 
//     println!("{some_int}")
// }

//problem
use std::io;

fn main(){ 
    // Takes the user input
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("failed to read the line");
    let word = first_word_fn(&input_string);
    println!("first word is: {word}");
}

fn first_word_fn(s :&str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){ 
        if item == b' '{ 
            return &s[0..i];
        }
    }
    &s[..]
}
