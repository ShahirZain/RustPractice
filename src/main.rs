use std::{io};
fn main() {
  /*  println!("Enter Number");
    let mut mynum = String::new();
    io::stdin().read_line(&mut mynum)
        .expect("Failed to read your input");

    let mynum: i32 = match mynum.trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    };
    */
    // println!("Input Radius is : {}",mynum);
    // println!("Area of Circle with radius {} is {}",mynum,area_of_circle(mynum));
    // check_num(mynum);
    // check_divisible();
    // println!("{}",volume_of_sphere(mynum));
    // copy_string();
    //even_odd(mynum);
    // check_vowel();
    // area_of_triangle();
    //euclidean_distan();
    //sum_of_npositive()
    decimal_to_binary();
    
}
/*
fn area_of_circle (x:i32) -> f32{
    let mut sq : f32 = x.pow(2)  as f32;
    
    return 3.14 * sq
}

*/
/*
fn check_num (z:i32){
    if z == 0 {
        println!("Number is zero");
    }
    else if z > 0 {
        println!("Number is positive");
    }
    else {
        println!("Number is negative");
    }
}
*/

/*
fn check_divisible (){
    println!("Enter numerator");
    let mut mynum = String::new();
    io::stdin().read_line(&mut mynum)
        .expect("Failed to read your input");

    let mynum: i32 = match mynum.trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    };
    println!("Enter Denominator:");
    let mut denominator = String::new();
    io::stdin().read_line(&mut denominator)
        .expect("Failed to read your input");

    let denominator: i32 = match denominator.trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    };

    if mynum % denominator == 0 {
        println!("Num is completely divisible by denominator");
    }
    else {
         println!("Num is not completely divisible by denominator");
    }
}

*/

/*
fn volume_of_sphere(x:i32) -> f32{
    return 1.33 * 3.14 * x.pow(3) as f32
} 
*/
/*
fn copy_string(){
    println!("Enter String");
    let mut my_string = String::new();
    io::stdin().read_line(&mut my_string)
        .expect("Failed to read your input");
    println!("How many copies of String you need?");
     let mut mynum = String::new();
    io::stdin().read_line(&mut mynum)
        .expect("Failed to read your input");

    let mynum: usize = match mynum.trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    };
    print!("{} copies of {}",mynum,my_string.repeat(mynum));
}
*/
/*
fn even_odd(x:i32) {
    if x % 2 == 0 {
        println!("num is even");
    }
    else  {
        println!("num is odd");
    }
}
*/
/*
fn check_vowel() {
    println!("Enter character");
    let mut my_string = String::new();
    io::stdin().read_line(&mut my_string)
        .expect("Failed to read your input");
        println!("{}",my_string);
    if my_string == "A".to_string() || my_string == "E" || my_string == "I" ||my_string ==  "O" ||my_string ==  "U" ||  my_string == "a" || my_string == "e" || my_string == "i" ||my_string ==  "o" ||my_string ==  "u"  {
        println!("String is vowel");
    }
    else{
         println!("String is not vowel");
    }
}
*/
/*
fn area_of_triangle(){

    println!("Enter base");
    let mut base = String::new();
    io::stdin().read_line(&mut base)
        .expect("Failed to read your input");

    let base: i32 = match base.trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    };
    println!("Enter altitue:");
    let mut altitue = String::new();
    io::stdin().read_line(&mut altitue)
        .expect("Failed to read your input");

    let altitue: i32 = match altitue.trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    };   
    println!("{}",0.5 * (base * altitue) as f32);
}

*/

/*
fn euclidean_distan(){

    println!("Enter x1");
    let mut x1 = String::new();
    io::stdin().read_line(&mut x1)
        .expect("Failed to read your input");

    let x1: i32 = match x1.trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    };
    println!("Enter y1:");
    let mut y1 = String::new();
    io::stdin().read_line(&mut y1)
        .expect("Failed to read your input");

    let y1: i32 = match y1.trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    };  

    
    println!("Enter x2");
    let mut x2 = String::new();
    io::stdin().read_line(&mut x2)
        .expect("Failed to read your input");

    let x2: i32 = match x2.trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    };
    println!("Enter y2:");
    let mut y2 = String::new();
    io::stdin().read_line(&mut y2)
        .expect("Failed to read your input");

    let y2: i32 = match y2.trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    }; 

    println!("Euclidean Distance is {}" , ((x2-x1).pow(2) + (y2-y1).pow(2)));
}
*/

/*
fn sum_of_npositive () {

     println!("Enter num :");
    let mut num  = String::new();
    io::stdin().read_line(&mut num )
        .expect("Failed to read your input");

    let num : i32 = match num .trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    }; 
    let mut sum = 0;
    for i in 1..=num {
        sum = sum + i ;
    }
    println!("Sum of N number is {}",sum);
}

*/
//not completed 
/*
fn decimal_to_binary(){
     println!("Enter num :");
    let mut num  = String::new();
    io::stdin().read_line(&mut num )
        .expect("Failed to read your input");

    let num : i32 = match num .trim().parse() {
        Ok(num) => num,
        Err(_) => 4
    }; 
    let mut remainder  = String::new();
    for i in 1..=num{
        
            println!("{}",(num% 2));
    
    }
}
*/
