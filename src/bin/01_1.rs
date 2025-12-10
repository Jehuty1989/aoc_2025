const INPUT: &str = include_str!("01_input_test.txt");
// const INPUT: &str = include_str!("01_input.txt");

fn main() {
    let mut start_position: i32 = 50;
    let mut zero_count: i32 = 0;

    for line in INPUT.lines() {
        let (direction, how_many_turns_str) = line.split_at(1);
        let how_many_turns: i32 = how_many_turns_str.parse::<i32>().unwrap() % 100;

        if direction.to_lowercase() == "l" {
            start_position -= how_many_turns;
        } else {
            start_position += how_many_turns;
        }

        if start_position > 99 {
            start_position -= 100;
        } else if start_position < 0 {
            start_position += 100;
        }

        if start_position == 0 {
            zero_count += 1
        }
    }

    println!("{}", zero_count)
}
