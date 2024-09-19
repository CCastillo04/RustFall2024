
//assignment 1
   fn fahrenheit_to_celsius(f: f64){

       let fc: f64 = f.clone();

       let _c: f64 = (fc-32.0)*(5.0/9.0);

       println!("was {} in fahrenheit but now {:.2} in Celsius",f, _c);
   }

   fn celsius_to_fahrenheit(c: f64){

       let cc: f64 = c.clone();

       let _f: f64 = (cc*(9.0/5.0))+32.0;

       println!("was {} in celsius {:.2} in Fahrenheit", c,_f);

   }
   fn main() {

   const NUM: f64 = 32.0;
   //declare constant
   let mut num: f64 = 33.0;
  //convert from f to c and c to f in two different functions
   let mut count: i32 = 0;
 
   println!("{} is a freezing point in fahrenheit!", NUM);

  //must have mutable variable -> convert to c
  //loop to convert the next 5 numbers

   loop{

       fahrenheit_to_celsius(num);

       num += 1 as f64;
       count += 1;

       if  count == 5{
           break;
       }
   }

  }

//assignment 2
  fn is_even(num: i32){
      if num == 0 {
          println!("{} is not even or odd", num);
          return;
      }
      if num % 2 == 0 {
          println!("{} is even", num);
          return;
      }

      println!("{} is odd", num);

      return;
  }

  fn main(){
  //array of 10 int to return a function if it is even or false
  let array: [i32;10] = [0,1,2,3,4,5,6,7,8,15];
  let mut store: i32 = array[0];

  let mut sum: i32 = 0;
  //we also want the sum of the total array
  //largest number
  let mut cnt: usize = 0;
  while cnt < array.len() {
      is_even(array[cnt]);

      if (array[cnt] % 3 == 0) && (array[cnt] % 5 == 0) {
          println!("fizzbuzz");
      }
      else if array[cnt] % 3 == 0 {
          println!("fizz");
      }
      else if array[cnt] % 5 == 0 {
          println!("buzz");
      }

      sum += array[cnt];

    
      if store < array[cnt] {
          store = array[cnt];
      }

      cnt+= 1;

  }
  println!("Total sum is: {}", sum);
  println!("Largest number is: {}", store)
 }

// //assignment 3 
// //store secret number (hard code)
// //check guess that returns 0 if correct, 1 if too high, -1 if too low
// //must track the number of loops it took to
// //}
fn check_guess(guess: i32, secret: i32)-> i32{

    if guess == secret{
        return 0;
    }else if guess > secret{
        return 1;
    }else{
        return -1;
    }

}
fn main(){

const SECRET: i32 = 24;

let mut guess: i32 = 12;
let mut _num: i32 = 0;

loop{

   let ans: i32 = check_guess(guess, SECRET);

    if ans == 0
    {
        println!("{} was correct!", guess);
        break;
    }else if ans == 1{
        println!("{} was too high", guess);
        guess -=1;
    }else{
        println!("{} was too low", guess);
        guess +=5;
    }

    _num+=1;
}

println!("Congrats, it took {} number of tries!", _num);

}
