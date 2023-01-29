fn main() {
//variables are by default immutable. 
//It is important to get compile time errors while we try to change the value of a variable which is by default immutable. 

let x = 6; //declaring an immutable variable

println!("The value of x: {}", x); 

 /* x = 5;  assigning a new value to x which is by default immutable

 This will cause an compile time error
 One way to fix this is explicitly declaring variable as mutable
 */

 let mut y = 5;
 println!("The value of y: {}", y);

 y = 6;
 println!("The updated value of y: {}", y);

/* Constants. Constants are immutable by default and cannot be changed in future. 
//constants are declared using const keyword. 
A constant can be only an expression but not a result.
we must annotate the type of constant we are declaring.
All constant variables are declared in uppercase.
*/

const HOUR_IN_SECONDS: u32 = 60 * 60; 

/* Shadowing
   we can declare a new variable with the same previous name. we can day first variable is shadowed by second variable. 

*/

let z = 6;
println!("value of z {}", z);
let z = z+1;
println!("value of z {}", z);
{
    let z = z*2;
    println!("value of z {}", z);
}
println!("value of z {}", z);




}
