fn main() {
    let ary = [0, 1, 2, 3];
    let ary_sliced = &ary[0..2];
    for aa in ary_sliced {
        println!("{}", aa);
    }
}