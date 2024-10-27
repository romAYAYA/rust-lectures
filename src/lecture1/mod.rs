pub fn say_hi() {
    let x = "privet";
    println!("Hello there {}", x);

    let x = 5;

    let check: i32 = -4;

    println!("check {}", check);

    println!("Hello there {}", x);

    let pi: f32 = -3.1415;

    println!("pi {}", pi);

    let some_float: f64 = 3.1234;
    let r: i32 = 5;

    let length: f64 = 2.0 + some_float * r as f64;

    println!("here is the zalupa {}", length);

    let check_sqrt = 1000;
    let sqrt = (check_sqrt as f64).sqrt() as i32;

    println!("check sqrt here {}", sqrt);

    let check_tuple: (i32, f64) = (10, 3.14);
    println!("check touple {:?}", check_tuple);

    let unit_type: () = ();

    println!("unit_type {:?}", unit_type);

    let (x, y) = check_tuple;

    println!("x y check {:?}", check_tuple);
    println!("x is {} y is {}", x, y);

    let ohoho = check_tuple.0;

    println!("checkcsakfdjlaskdgjsfdl {}", ohoho);

    let mut array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("array {:?}", array);

    let all_zeros: [i32; 10] = [0; 10];

    println!("all_zeros {:?}", all_zeros);

    let arr_length: usize = array.len();

    println!("arr length {}", arr_length);

    let x_arr = array[0];

    println!("x arr is {}", x_arr);

    array[3] = 6;
    println!("array 3 changed to {}", array[3]);

    println!("we are checking fn {}", check_fn_sum(5, 16));

    println!("wowooooowoowowowwowowo {}", do_some_magic());

    let check_state = do_some_magic(); // statement

    println!("here is the check state {}", check_state);

    let code_block = {
        let y = 5;
        y
    };

    println!("code block {}", code_block);

    if x > check {
        println!("x greater");
    } else if x < check {
        println!("check greater");
    } else {
        println!("maaaaagic");
    }

    let x = if x > check {
        println!("kavoo")
    } else {
        println!("chavoooo")
    }; // if как expression
    

    let mut i = 0;


    while i < 32 {
        i += 1;

        if i == 12 || i == 13 {
            continue;
        }

        println!("i {}", i);
    };

}

fn check_fn_sum(arg1: i32, arg2: i32) -> i32 {
    return arg1 + arg2;
}

fn do_some_magic() -> i32 {
    0
}

