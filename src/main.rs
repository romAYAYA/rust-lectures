mod lecture1;

fn main() {
    lecture1::say_hi();

    let arr = [1, 2, 43, 4, 5, 6, 7, 324, 45, 45];

    println!("asdf {:?}", lecture1::find(&arr, 45));
}
