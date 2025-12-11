// const INPUT: &str = include_str!("02_input_test.txt");
const INPUT: &str = include_str!("02_input.txt");

fn main() {
    let mut matches: Vec<u64> = [].to_vec();

    for range in INPUT.split(",") {
        let mut range_split = range.split('-');
        let from: u64 = range_split.next().unwrap().parse().unwrap();
        let to: u64 = range_split.next().unwrap().parse().unwrap();

        for number in from..=to {
            let number_string = number.to_string();
            let (first_half, second_half) = number_string.split_at(number_string.len() / 2);

            if first_half == second_half {
                matches.push(number);
            }
        }
    }

    let total: u64 = matches.iter().sum();

    println!("total: {}", total)
}
