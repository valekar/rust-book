use std::thread;
use std::time::Duration;
/**
    Closures
*/

fn main() {
    let intensity_1: u32 = 7;
    let random_number: u32 = 10;

    generate_workout_plan(intensity_1, random_number);
}

fn simulate_expensive_calculation(instensity: u32) -> u32 {
    println!("Calculating slowly");
    thread::sleep(Duration::from_secs(2));
    instensity
}

fn generate_workout_plan(instensity: u32, random_number: u32) {
    let mut cacher = Cacher::new(|num| {
        println!("Calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if instensity < 25 {
        println!("Today do {} pushups", cacher.value(instensity));
        println!("Next do {} situps", cacher.value(instensity));
    } else {
        if random_number == 3 {
            println!("Take a break, no exercise, njaay");
        } else {
            println!("Today, run for {} minutes", cacher.value(instensity));
        }
    }
}

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
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: None,
        }
    }

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
