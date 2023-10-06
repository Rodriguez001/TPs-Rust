fn factorial(num: u32) -> u32 {
    let mut result = 1;
    for i in 1..num {
        result = result * i ; 
    }
    return result;    
}

fn main() {
    let a = 10;
    let x = factorial(a);
    println!("The value of {} factorial is {} ", a, x);
}