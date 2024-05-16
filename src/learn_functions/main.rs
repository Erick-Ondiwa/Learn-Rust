//A program to compute the area of a rectangle using function
  //1. Functions without parameters
fn rectangle_area(length: i32, width: i32) -> i32{
   length * width
  
}

//2. Functions requiring parameters
fn circle_area(){
  const PI: f32 = 3.142;
  let radius = 2.8;

  let area = PI * (radius * radius);
  println!("The area of the circle is: {}", area);
} 
fn main (){
  //calling the rectangle_area function to execute
  circle_area();
  let area = rectangle_area(5,4); 

  println!("The area of the rectangle is: {}", area);
}