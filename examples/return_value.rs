use nome::func;

func!($ sum, |x: u32, y: u32| -> u32 {
    x + y
});

fn main() {
    println!(
        "sum({first}, {second}) = {sum}",
        first = 17,
        second = 25,
        sum = sum!(x = 17, y = 25)
    );

    println!(
        "sum({first}, Default) = {sum}",
        first = 17,
        sum = sum!(x = 17)
    );

    println!(
        "sum(Default, {second}) = {sum}",
        second = 25,
        sum = sum!(y = 25)
    );

    println!("sum(Default, Default) = {sum}", sum = sum!());
}
