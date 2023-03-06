fn internal_sum_function(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

pub fn partial_sum_1_and_2() {
    let sum_1_and_2 = |x| internal_sum_function(1, 2, x);
    println!("Sum 1, 2 and {}, equals: {}", 0, sum_1_and_2(0));
    println!("Sum 1, 2 and {}, equals: {}", 3, sum_1_and_2(3));
}
