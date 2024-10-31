fn main() {
    let x = 1;
    println!("{}", x);
    {
        let x = 2;
        println!("{}", x);
    }
    println!("{}", x)
}