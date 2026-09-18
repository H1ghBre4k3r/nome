use nome::func;

func!($ foo, |x: u32, y: u64| {
    println!("x = {x}, y = {y}")
});

fn main() {
    foo!();
    foo!(x = 5);
    foo!(y = 7);
}
