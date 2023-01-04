#[derive(PartialEq)]
enum State {
    Crates,
    Empty,
    Instructions,
}

fn part_1(input: &str) -> usize {
    _ = input;

    let mut crate_map: Vec<Vec<Option<u8>>> = vec![];

    let mut state = State::Crates;
    for line in input.lines() {
        match state {
            State::Crates => {
                // change state
                if !line.contains('[') {
                    state = State::Empty;
                    continue;
                }
                // parse and collect
                let mut fakeline: Vec<u8> = Vec::from(line);
                fakeline.insert(0, b' ');
                fakeline.insert(0, b' ');
                let tmp: Vec<Option<u8>> = fakeline
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| {
                        if (i + 1) % 4 == 0 {
                            return true;
                        }
                        false
                    })
                    .map(|(_, v)| match v {
                        b' ' => None,
                        _ => Some(*v),
                    })
                    .collect();
                crate_map.push(tmp)
            }
            State::Empty => {
                // reverse the crates so we can push values later if necessary
                crate_map.reverse();
                state = State::Instructions;
            }
            State::Instructions => {
                println!("instructions -- {}", line);
                let cmd: Vec<usize> = line.split(" ").filter(|v|{
                    match *v {
                        "move" | "from" | "to" => false,
                        _ => true,
                    }
                }).map(|v| v.parse().unwrap())
                .collect();

                let num = cmd[0];
                let src = cmd[1];
                let dst = cmd[2];

                let mut slots = 0;
                let mut first_dst = 0;
                for i in 0..crate_map.len(){
                    if crate_map[i][dst].is_none() {
                        slots += 1;
                    } else {
                        first_dst += 1;
                    }
                }

                while slots < num {
                    crate_map.push(vec![None; crate_map[0].len()]);
                    slots += 1
                }

                let mut first_src = 0;
                for i in 0..crate_map.len(){
                    if crate_map[i][src].is_some(){
                        first_src += 1;
                    } else {
                        break;
                    }
                }

                println!("line: {line}");
                println!("first_src: {first_src}");
                println!("first_dst: {first_dst}");
                println!("{:?}", crate_map);
                return 0;

            }
        }
    }

    println!("{}",char::from(crate_map[1][0].unwrap()));

    0
}

fn part_2(input: &str) -> usize {
    _ = input;

    0
}

mod test {

    #[allow(dead_code)]
    static INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_1() {
        assert_eq!(2, super::part_1(INPUT));
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(4, super::part_2(INPUT));
    // }
}

fn main() {
    let payload = include_str!("../input.txt");
    println!("num: {}", part_1(payload));
    println!("num: {}", part_2(payload));
}
