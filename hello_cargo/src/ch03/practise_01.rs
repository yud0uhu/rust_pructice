fn main(){
    let celsius_to_fahrenheit = celsius_to_fahrenheit(20.0);
    println!("セ氏20.0℃:華氏{}℃", celsius_to_fahrenheit);
    let fibonacci = fibonacci(4);
    println!("フィボナッチ数列の4項目:{}",fibonacci);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return celsius * 1.8 + 32.0;
}

fn fibonacci(n: i64) -> i64 {
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    }
   return n;
}
