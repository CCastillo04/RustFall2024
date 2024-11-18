use std::{thread,time::Duration};

fn track_changes(){
    let mut tracker =0;
    let mut update =||{
        tracker += 20;
        println!("Has updated to: {}", tracker);
    };
    update();
    update();
}

fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where 
    F: Fn(i32) -> i32,
    {
        let mut result = Vec::new();
        for x in vec{
            result.push(f(x));
        }
        result
    }

struct ComputeCache<T>
where 
    T: Fn() -> String,
    {
        computation: T,
        value: Option<String>,
    }

impl<T> ComputeCache<T>
where 
    T: Fn() -> String,
    {
        fn new(computation: T) -> Self{
            ComputeCache{
                computation,
                value: None,
            }
        }

        fn get_value(&mut self) -> String{
            match &self.value {
                Some(v) => {
                    println!("Retrieved from cache instantly");
                    v.clone()
                }

                None => {
                    thread::sleep(Duration::from_secs(1));
                    let v = (self.computation)();
                    self.value = Some(v.clone());
                    v
                }
            }
        }
    }

fn main(){
    //task 1
    //let add = |x: i32, y: i32| x + y;
    //println!("5 + 3 = {}", add(5, 3)); // Output: 5 + 3 = 8

let multi = |x:i32, y:i32| x*y;
println!("5 * 10 = {}", multi(5,10));

    //task 2
    //let mut total = 0;
    //let mut accumulate = || {
    // total += 5
    // println!("Total: {}", total);
    //};
    //accumulate(); // Output: Total: 5
    //accumulate(); // Output: Total: 10

track_changes();

    //task 3
    let numbers = vec![1,2,3];

    let double = process_vector_with_for_loop(numbers.clone(), |x|{
x*2
    });

    let replace = process_vector_with_for_loop(numbers.clone(), |mut x|{
        if x > 2{
            x = 0;
        }
        x
    });

println!("Doubled: {:?}",double);
println!("Replaced: {:?}",replace);

    //task 5

    let mut cache = ComputeCache::new(||{
        println!("Computing... this will take 2 seconds...");
        thread::sleep(Duration::from_secs(1));
        "Hello world!!".to_string()
    });

    println!("First call");
    println!("Result: {}", cache.get_value());

    println!("\nSecond call");
    println!("Result (cache): {}", cache.get_value());
}