use std::f64;

fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

fn percentage(number: i64, ratio: i64) -> i64 {
    return number / ratio;
}

fn fibonacci(number: i64) -> i64 {
    if number <= 1 {
        return number;
    }

    return fibonacci(number - 1) + fibonacci(number - 2);
}

fn compound_interest(capital: f64, annual_rate: f64, periods_by_year: i64, years: i64) -> f64 {
    let n: f64 = periods_by_year as f64;
    let t: f64 = years as f64;
    return capital * (1.0 + annual_rate / n).powf(n * t);
}

fn main() {
    println!("[function] add result={}", add(10, 20));
    println!("[function] percentage result={}", percentage(500, 10));
    println!("[function] fibonacci result={}", fibonacci(10));
    println!(
        "[function] compound interest | capital=100 | result={:>.2}",
        compound_interest(100.0, 0.08, 12, 1)
    )
}
