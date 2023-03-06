mod func_paradigm;

use func_paradigm::closure_output;
use func_paradigm::closure_params;
use func_paradigm::func_params;
use func_paradigm::iterators;
use func_paradigm::partial_application;

fn example_transform(input: &i32) -> f32 {
    *input as f32 / 2.0
}

fn example_filter(input: &f32) -> bool {
    *input != 0.0
}

fn func_param_example() {
    let output = func_params::filter_and_transform(example_transform, example_filter);
    output
        .iter()
        .for_each(|x| println!("Output of func_params: {}", x));
}

fn func_closure_example_01() {
    let output = closure_params::filter_and_transform(example_transform, example_filter);
    output
        .iter()
        .for_each(|x| println!("Output of closure_params 1: {}", x));
}

fn func_closure_example_02() {
    let divide_by = 2.0;
    let shouldnt_be_equal = 0.0;
    let output = closure_params::filter_and_transform(
        |i| *i as f32 / divide_by,
        |f| *f != shouldnt_be_equal,
    );
    output
        .iter()
        .for_each(|x| println!("Output of closure_params 2: {}", x));
}

fn return_closure() {
    let my_closure = closure_output::return_closure(3);
    println!("is 3 equal 3?: {}", my_closure(3));
    println!("is 4 equal 3?: {}", my_closure(4));
}

fn main() {
    iterators::basic();
    println!("=====================");
    func_param_example();
    println!("=====================");
    func_closure_example_01();
    println!("=====================");
    func_closure_example_02();
    println!("=====================");
    return_closure();
    println!("=====================");
    partial_application::partial_sum_1_and_2();
}
