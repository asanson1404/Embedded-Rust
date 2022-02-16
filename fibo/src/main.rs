fn main() {
    println!("Hello, world!");
    for i in 0..=42{
        println!("fibo[{}] = {}", i, fibo(i));
    }
}

fn fibo(n: u32) -> u32 {

    if n == 0 {0} else if n == 1 {1}
    else {fibo(n-1) + fibo(n-2)}

    // Autre possibilité 
    /*match n {
        0   => 0,
        1   => 1,
        _   => fibo(n-1) + fibo(n-2)
    }*/
}
