const INPUT: &str = include_str!("01_input.txt");
// const INPUT: &str = include_str!("01_test_input.txt");
// const INPUT: &str = include_str!("01_custom.txt");

fn main() {
    let mut start_position: i32 = 50;
    let mut zero_count: i32 = 0;

    for line in INPUT.lines() {
        let before_turning = start_position;
        let (direction, how_many_turns_str) = line.split_at(1);
        let how_many_turns: i32 = how_many_turns_str.parse::<i32>().unwrap();
        let how_many_turns_by_100 = how_many_turns / 100;
        zero_count += how_many_turns_by_100;

        println!("how_many_turns_by_100: {}", how_many_turns_by_100);

        let how_many_turns_mod_100 = how_many_turns % 100;

        println!("before turning: {}", before_turning);
        println!("how_many_turns: {} {}", direction, how_many_turns);

        if direction.to_lowercase() == "l" {
            start_position -= how_many_turns_mod_100;
        } else {
            start_position += how_many_turns_mod_100;
        }

        println!("after turning: {}", start_position);

        if start_position > 99 {
            if before_turning != 0 {
                zero_count += 1;
            }
            start_position -= 100;
        } else if start_position < 0 {
            if before_turning != 0 {
                zero_count += 1;
            }
            start_position += 100;
        } else if start_position == 0 && how_many_turns_by_100 < 1 {
            zero_count += 1;
        }

        // if start_position == 0 {
        //     zero_count += 1;
        // }
        println!(
            "start_position: {}, zero_count: {}",
            start_position, zero_count
        );
        println!("----------------------");
    }

    println!("{}", zero_count);
}
