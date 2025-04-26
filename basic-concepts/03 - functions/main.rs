// fn f(x: i32) -> i32 { x + 1 }
//
// fn main() {
//
//     println!("{}", f({
//
//         let y = 1;
//
//         y + 1
//
//     }));
//
// }
// gcd
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 &&  m != 0); 
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n   //  return n
}


fn main() {
    println!("{}", gcd(50,25));
}