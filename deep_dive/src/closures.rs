use std::{thread, time::Duration};

fn run() {
    // like functions, but can be passed as variables
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity);
}

fn expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// memoization
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // constructor
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
        }
    }

    // method
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32) {
    // anonymous function, |input| { body }
    let mut cached_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    let exmple_closure = |x: String| x;

    if intensity < 25 {
        // call on demand
        cached_result.value(intensity);
    } else if intensity < 50 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
    }
}
