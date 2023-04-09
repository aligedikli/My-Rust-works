#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn bitwise_logic(){
let c: i32 = 1|2;  //   | -> veya
//& AND
//^ XOR 
//1|2 1 =01
//2 ->10
//01 | 10 =11

println!("1|2 = {}",c);

//logical işlemler
let pi_kucukmu_4den :bool = std::f64::consts::PI<4.0;
// mantıksal olarak > >= < <= == != sorgulaması yapabiliriz
println!("sonuc {}", pi_kucukmu_4den);

let x:i32=5;
let x_esit_5 :bool = x!=5;
println!("eşitlik : {}", x_esit_5);

}

