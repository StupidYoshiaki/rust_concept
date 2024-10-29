#[path = "module_hello.rs"]
mod module_hello;

fn main() {
    module_hello::print_hello();
}