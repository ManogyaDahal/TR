// 1.1 function for hello world
// fn hw(){ 
//    println!("Hello world")
// }


// 1.2 var declaration
// fn var(){
//   let mut x : u8 = 13;
//   println!("{}",x);
//
//   let x : f32 = 1.232;
//  println!("{}",x);
//
//  let x; 
//  x=10; 
// }

// 1.3 datatypes
// fn Datatypes(){ 
//     let y = 14; //i32 by default
//     let x : i64 = 14; //int
//     let u : char= 'w'; //unicode
//     let _bv= true; //boolean value
//     let _r_tuple = (13, false, "something");
//
//
//     println!("i32:{},\n i64:{}, char:{}, tuple0:{}, tuple1:{}", y, x, u,_r_tuple.0, _r_tuple.2)
// }

//1.4 Basic type conversion
// fn typeConversion(){ 
//     let x = 12u8; //unsigned 8 bit
//     let y = 123u16; //unsigned 16 bit
//     let z = x as u32 + y as u32;
//
//     println!("x:{}, y:{}, z:{}", x, y, z);
//
//     let t : bool = false; 
//     println!("t:{}", t as u32) //boolean to unsigned int conversion 
// }

//1.5 constants
// const PI: f32 = 3.14; 
// fn main(){ 
//     println!("The value of PI is {}", PI);
// }

//1.6 Arrays
// fn arrays() {
//    let num: [u32; 3] = [1, 2, 3];
//    println!("arr:{:?}", num);
//    println!("arr:{}", num[2]);
//    //> dynamic arrays are talked about in vector chapter
// }

//1.7 functions
// fn main() {
//     println!("42 + 13 = {}", add(42, 13));
//     println!("42 - 13 = {}", subtract(42, 13));
// }
//
// fn add( x: i32, y: i32) -> i32{ 
//     return x+y
// }
// fn subtract(x:i32, y:i32) -> i32{ 
//     x-y
// }

// 1.8 and 1.9 multiple returns and no returns
fn main(){ 
    let result = swap(2, 3);
    println!("1:{}, 2:{}", result.0, result.1);

    //destructuring a tuple
    let (a,b) = swap(result.0, result.1);
    println!("1:{}, 2:{}", a,b);

    println!("noting:{:?}, nothing2:{:?}", make_nothing(), make_nothing2());
}
fn make_nothing()->(){} //these functions just exists
fn make_nothing2(){}

fn swap(x:i32, y:i32) -> (i32, i32){ 
    return(y,x)
}
