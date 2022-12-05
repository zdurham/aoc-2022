fn parse_hand(hand: &str) -> i32 {
  match hand {
    "X" => 0,
    "A" => 0,
    "B" => 1,
    "Y" => 1,
    "C" => 2,
    "Z" => 2,
    _ => panic!("not a hand")
  }
}

pub fn day2() {
  let input = include_str!("./input.txt");

  let games: Vec<Vec<&str>> = input.lines().map(|line| line.split(" ").collect()).collect();

  let score = games.iter().map(|game| {
    let opp_hand = parse_hand(game[0]);
    let result = parse_hand(game[1]);

    let my_hand = (opp_hand + result - 1).rem_euclid(3);
    if ((opp_hand + 1) % 3) == my_hand { 
      return 6 + my_hand + 1
    } else if opp_hand == my_hand {
      return 3 + my_hand + 1
    } else {
      return 0 + my_hand + 1
    }
  }).sum::<i32>();

  println!("{}", score);
}
