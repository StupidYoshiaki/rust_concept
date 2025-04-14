fn pick2(x: &[i32], y: &[i32]) -> (&[i32], &[i32]) {
    (&x[..end], &y[..end])
}

fn main() {
    let x = 1;
    println!("{}", x);
    {
        let x = 2;
        println!("{}", x);
    }
    println!("{}", x)
}
