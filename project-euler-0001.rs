// Enter your code here 
use std::io;
fn mult(max: u64, n: u64) -> u64 {
    /* 
    for i in (1..m).filter(|x| x % 3 == 0 || x % 5 == 0) {
        sum += i;
    }
    */
    let ct: u64 = (max - 1) / n;
    let sum: u64 = (1 + ct) * ct / 2 * n;
    return sum;
}

fn main () {
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).ok().expect("read_line error");
    let n1: u32 = s1.trim().parse().ok().expect("first line of input format error");
    
    for _ in 0..n1 {
        let mut s2 = String::new();
        io::stdin().read_line(&mut s2).ok().expect("read_line error");
        let n2: u64 = s2.trim().parse().ok().expect("input data format error");
        println!("{}", mult(n2, 3) + mult(n2, 5) - mult(n2,15));
    }
}
