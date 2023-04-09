#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main () {
 let a:u8 = 125;
 println!("a = {}" , a);
 
 let nut b:i8 = 0;

 println!("önce b = {}" , b);
 b = 22;
 println!("sonra b = {}", b);

 let c:i32 = 123456789; //i32 == 32 bit = 4 byte
 println!(" c = {} ve boyutu {} byte tır.", c,mem::size_of_val(val:&c));

}