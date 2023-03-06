pub fn return_closure(i: i32) -> impl Fn(i32) -> bool {
    move |a| a == i
}
