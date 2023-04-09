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
 
 let d:isize = -200;
 let d_boyut :usize = mem::size_of_val(val:&d);
 println!("d = {} ve boyutu {} byte, bilgisayarınız {} bit mimariye sahiptir", d,d_boyut,d_boyut*8);

 let e:char = 'g';
 println!(" e = {} ve boyutu {} byte tır.", e,mem::size_of_val(val:&e));

 
 let mut f:f32 = 2.0000000005;
 println!(" f = {} ve boyutu {} byte tır.", f,mem::size_of_val(val:&f));
 
 let g:bool = false;
 println!(" g = {} ve boyutu {} byte tır.", g,mem::size_of_val(val:&g));



}