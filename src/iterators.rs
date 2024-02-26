
fn iterator_fun() {

    let strings = vec!["John", "Wick"];
    let caps: Vec<String> = strings.iter()
        .map(|&s| s.to_uppercase())
        .collect();

    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let even_sum = v.iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * 3 + 1)
        .reduce(|sum, x| sum + x);


}