pub fn basic() {
    let initial_list = vec![1, 2, 3, 4, 5, 6];

    let list = initial_list
        .iter()
        .map(|i| i + 3)
        .filter(|i| i % 2 == 0)
        .collect::<Vec<_>>();
    list.iter()
        .for_each(|x| println!("Basic iterator output: {}", x));

    let overly_complicated_sum = list.iter().fold(0, |acc, i| acc + i);
    println!("Sum of the above is: {}", overly_complicated_sum);
}
