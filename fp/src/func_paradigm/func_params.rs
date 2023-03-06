pub fn filter_and_transform(
        transform: fn(&i32) -> f32,
        filter: fn(&f32) -> bool
) -> Vec<f32> {
    let initial = vec![0, 1, 2, 3, 4, 5, 6, 7];
    initial.iter()
        .map(transform)
        .filter(filter)
        .collect()
}
