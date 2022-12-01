pub fn day1() {
    let string = include_str!("./input.txt");
    let mut totals: Vec<i32> = vec![];
    let mut total: i32 = 0;
    
    let split = string.split("\n");
    for x in split {
        let x = x.parse::<i32>();
        if x.is_ok() {
            total += x.unwrap();
        } else {
            totals.push(total);
            total = 0;
        }
    }

    println!("{}", totals.iter().max().unwrap());

    // part 2
    totals.sort();
    totals.reverse();
    println!("{}", totals[0..3].iter().sum::<i32>());
}
