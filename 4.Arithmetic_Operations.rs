#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn veriTipleri () {...}

fn aritmetikIşlemler (){
 let mut a:i32 = 2+5*3;
 println!("sonuc a : {}",a);
 a=a+1;
 a+=1;

 println!(" {} / {} işleminden kalan ={}",a,4,(a%4));

 let a_kupu :i32 = i32::pow(self: a, exp:3);
 println! ("a değerinin küpü {}", a_kupu);

 let b:f64 = 2.6
 let b_kupu :f64 = f64::powi(self: b, n:3);
 let b_ustu_pi :f64 = f64::powf(self b, n: std::f64::consts::PI);
 println!("{0} üstü 3 : {1} ve {0} üstü pi :{2}",b,b_kupu,b,b_ustu_pi );
 

}