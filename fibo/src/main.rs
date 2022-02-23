use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of fibo values to print
    #[clap(short = 'c', long = "count")]
    count: u32,
}

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
    for i  in 0..=args.count {
        if fibo(i) == None {break;} 
        else {println!("fibo[{}] = {}", i, fibo(i).unwrap());}
    }
}

fn fibo(n: u32) -> Option<u32> {

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

    let mut ret: Option<u32> = Some(1);
    let mut previous1: Option<u32> = Some(0); 
    let mut previous2: Option<u32>;

    if      n == 0 {Some(0)}
    else if n == 1 {Some(1)}
    else {
        for _ in 2..=n {

            previous2 = previous1;
            previous1 = ret;

            //Arithmétique saturée
            //ret = previous1.saturating_add(previous2);

            //Arithmétique vérifiée
            //ret = previous1.checked_add(previous2).unwrap();

            //Retourner None si ret tient pas sur 32 bits
            ret = previous1.unwrap().checked_add(previous2.unwrap());
            match ret {
                None => break,
                Some(_y) => continue      
            }
        }
        ret
    }
}
