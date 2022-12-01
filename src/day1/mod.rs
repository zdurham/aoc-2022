use std::fs;

fn read_file() -> Vec<&'static str> {
    let file_path = "./src/day1/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let array_of_calories = contents.lines().map(|s| s.trim().map(String::from)).collect::<Vec<String>>()
}

pub fn day1() {
    let mut most_calories = 0;
    let mut current_elf_calories = 0;
    let elf_calories = read_file();
    for calories in elf_calories {
         match calories {
            "" => {
                if current_elf_calories > most_calories {
                    most_calories = current_elf_calories;
                    current_elf_calories = 0;
                }
            },
            _ => {
                current_elf_calories += calories.parse::<i32>().expect("whoa there not a number")
            }
        }
    }

    println!("{}", most_calories)
}
