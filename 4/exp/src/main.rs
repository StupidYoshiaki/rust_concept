enum EnumExample {
    TupleTypeExample1(String),
    TupleTypeExample2(i32, bool),
    StructTypeExample{name: String, age: u8},
}

fn main() {
    let mut v: Vec<EnumExample> = Vec::new();

    v.push(EnumExample::TupleTypeExample1(String::from("Hello")));
    v.push(EnumExample::TupleTypeExample2(10, true));
    v.push(
        EnumExample::StructTypeExample{
            name: String::from("taro"),
            age: 10,
        }
    );

    for e in &v {
        if let EnumExample::StructTypeExample{name: n, age: a} = e {
            println!("StructTypeExample_iflet: name = {}, age = {}", n, a);
        }
    }

    for e in &v {
        match e {
            EnumExample::TupleTypeExample1(s) => {
                println!("TupleTypeExample1: s = {}", s);
            }
            EnumExample::TupleTypeExample2(n, b) => {
                println!("TupleTypeExample1: n = {}, b = {}", n, b);
            }
            EnumExample::StructTypeExample{name: n, ..} => {
                println!("StructTypeExample: name = {}", n);
            }
        }
    }
}