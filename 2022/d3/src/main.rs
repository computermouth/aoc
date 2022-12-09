#![feature(iter_next_chunk)]

fn get_sum(input: &str) -> usize {
    let mut sum: usize = 0;

    for line in input.lines() {
        let len = line.len() / 2;

        let sack1 = line[0..len].as_bytes();
        let sack2 = line[len..].as_bytes();

        for c in sack1 {
            if sack2.contains(c) {
                sum += match c.is_ascii_uppercase() {
                    true => (c - b'A' + 27) as usize,
                    false => (c - b'a' + 1) as usize,
                };
                break;
            }
        }
    }

    sum
}

fn get_sum_part2(input: &str) -> usize {
    let mut sum: usize = 0;

    for chunk in input.lines().collect::<Vec<_>>().chunks(3) {
        println!("chunk: {:?}", chunk);
        for c in chunk[0].as_bytes() {
            if chunk[1].as_bytes().contains(c) && chunk[2].as_bytes().contains(c) {
                sum += match c.is_ascii_uppercase() {
                    true => (c - b'A' + 27) as usize,
                    false => (c - b'a' + 1) as usize,
                };
                break;
            }
        }
    }

    sum
}

mod test {

    static INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        assert_eq!(super::get_sum(INPUT), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::get_sum_part2(INPUT), 70);
    }
}

fn main() {
    let payload = include_str!("../input");
    println!("score: {}", get_sum(payload));
    println!("score_part2: {}", get_sum_part2(payload));
}
