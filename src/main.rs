fn main() {
    let x: i32 = 144;
    let y: i32 = 32;
    let z: u8 = 22;
    println!("x: {}, y: {}, z {}", x, y, z);

    let _ismale = true;
    let _isabove18 = false;

    if _ismale {  // this is how if is written in rust
        println!("You are Male")
    } else {
        print!("you are female");
    }

    if _ismale && _isabove18 {
        println!("You are an adult Male");
    } else {
        println!("you are not an adult male");
    }
    let name = String::from("Pawan Mishra");
    println!("{}", name);

    let mut sum = 0;

    for i in 1..100 {  // this is rust for loop
        sum = sum + i;
    }
    println!("{}", sum);

    fn dosum(a: i32,b: i32) -> i32 { // this is how you return in a fn
        return a+b;
    }

    let a = 34;
    let b = 44;
    let sum = dosum(a,b);

    println!("Sum of {},{} is {}" , a,b,sum);
}
