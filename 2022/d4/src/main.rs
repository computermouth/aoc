#[derive(Debug)]
struct Elf {
    start: isize,
    end: isize,
}

fn part_1(input: &str) -> usize {
    _ = input;
    let mut total = 0;
    let elfpairs = input
        .lines()
        .map(|line| {
            line.split(',')
                .flat_map(|e| e.split_once('-'))
                .map(|s| Elf {
                    start: s.0.parse().unwrap(),
                    end: s.1.parse().unwrap(),
                })
                .collect::<Vec<Elf>>()
        })
        .collect::<Vec<Vec<Elf>>>();

    for pair in elfpairs {
        if ((pair[0].start >= pair[1].start) && (pair[0].end <= pair[1].end))
            || ((pair[1].start >= pair[0].start) && (pair[1].end <= pair[0].end))
        {
            total += 1;
        }
    }

    total
}

fn part_2(input: &str) -> usize {
    _ = input;
    let mut total = 0;
    let elfpairs = input
        .lines()
        .map(|line| {
            line.split(',')
                .flat_map(|e| e.split_once('-'))
                .map(|s| Elf {
                    start: s.0.parse().unwrap(),
                    end: s.1.parse().unwrap(),
                })
                .collect::<Vec<Elf>>()
        })
        .collect::<Vec<Vec<Elf>>>();

    for pair in elfpairs {
        if ((pair[0].start >= pair[1].start) && (pair[0].start <= pair[1].end)
            || (pair[0].end >= pair[1].start) && (pair[0].end <= pair[1].end))
            || ((pair[1].start >= pair[0].start) && (pair[1].start <= pair[0].end)
                || (pair[1].end >= pair[0].start) && (pair[1].end <= pair[0].end))
        {
            total += 1;
        }
    }

    total
}

mod test {

    #[allow(dead_code)]
    static INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_1() {
        assert_eq!(2, super::part_1(INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(4, super::part_2(INPUT));
    }
}

fn main() {
    let payload = include_str!("../input");
    println!("num: {}", part_1(payload));
    println!("num: {}", part_2(payload));
}
