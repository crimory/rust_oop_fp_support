pub fn filter_and_transform<T, F>(
        transform: T,
        filter: F
) -> Vec<f32>
where
    T: Fn(&i32) -> f32,
    F: Fn(&f32) -> bool,
{
    let initial = vec![0, 1, 2, 3, 4, 5, 6, 7];
    initial.iter()
        .map(transform)
        .filter(filter)
        .collect()
}
