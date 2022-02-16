fn main() {
    println!("Hello, world!");
    for i in 0..=42{
        println!("fibo[{}] = {}", i, fibo(i));
    }
}

fn fibo(n: u32) -> u32 {

    //=========================
    // IMPLEMENTATION RECURSIVE
    //=========================
    //if n == 0 {0} else if n == 1 {1}
    //else {fibo(n-1) + fibo(n-2)}

    // Autre possibilité 
    /*match n {
        0   => 0,
        1   => 1,
        _   => fibo(n-1) + fibo(n-2)
    }*/

    //=========================
    // IMPLEMENTATION ITERATIVE
    //=========================
    let mut val: u32 = 0;
    let mut previous1:u32; 
    let mut previous2:u32;

    if      n == 0 {return 0;}
    else if n == 1 {return 1;}
    else {
        for _ in 2..=n {
            previous1 = 1;
            previous2 = 
        }
    }

}
