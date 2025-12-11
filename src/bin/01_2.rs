const INPUT: &str = include_str!("01_input.txt");
// const INPUT: &str = include_str!("01_input_test.txt");
// const INPUT: &str = include_str!("01_input_custom.txt");
const START_POSITION: i32 = 50;

/**
 * This solution is not performant at all
 */
fn main() {
    let mut current_position: i32 = START_POSITION;
    let mut zero_count: i32 = 0;

    for line in INPUT.lines() {
        let (direction, how_many_turns_str) = line.split_at(1);
        let mut how_many_turns: i32 = how_many_turns_str.parse::<i32>().unwrap();
        println!("starting_position: {}", current_position);
        println!(
            "direction: {}, how_many_turns_str: {}",
            direction, how_many_turns_str
        );

        while how_many_turns > 0 {
            if direction == "L" {
                current_position -= 1;
            } else {
                current_position += 1;
            }

            if current_position < 0 {
                current_position += 100;
            } else if current_position > 99 {
                current_position -= 100;
            }

            if current_position == 0 {
                zero_count += 1;
            }

            how_many_turns -= 1;
        }

        println!("current_position: {}", current_position);
        println!("zero_count: {}", zero_count);
        println!("----------------------");
    }

    println!("{}", zero_count);
}
