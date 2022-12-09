static PAYLOAD: &'static str = include_str!("../input");

static ROCK: isize = 0b001;
static PAPR: isize = 0b010;
static SCIS: isize = 0b100;

fn get_score_part2(s: &str) -> isize {

    let mut score: isize = 0;

    for line in s.lines() {

        let p2 = match line.chars().nth(0).unwrap() {
            'A' => ROCK,
            'B' => PAPR,
            'C' => SCIS,
            _ => todo!()
        };

        let mut p1: isize = match line.chars().nth(2).unwrap() {
            'X' => {
                let mut p1 = p2 >> 1;
                if p1 == 0b000 {p1 = 0b100};
                p1
            }, // lose
            'Y' => {score += 3; p2},            // tie 
            'Z' => {
                score += 6;
                let mut p1 = (p2 << 1);
                if p1 == 0b1000 {p1 = 0b001};
                p1 // win 
            },
            _ => todo!()
        };
        
        if p1 == 4 { p1 = 3 };
        score += p1;

    }

    score
}

#[test]
fn test_get_score2(){
    let input = "A Y
B X
C Z";

    assert_eq!(get_score_part2(input), 12)

}

fn get_score(s: &str) -> isize {

    let mut score: isize = 0;

    for line in s.lines() {

        let p2 = match line.chars().nth(0).unwrap() {
            'A' => ROCK,
            'B' => PAPR,
            'C' => SCIS,
            _ => todo!()
        };
        let mut p1= match line.chars().nth(2).unwrap() {
            'X' => ROCK,
            'Y' => PAPR,
            'Z' => SCIS,
            _ => todo!()
        };

        if p1 >> 1 == p2 || p1 << 2 == p2 { // win
            score += 6;
        } else if p1 == p2 { // tie
            score += 3;
        }
        
        if p1 == 4 { p1 = 3 };
        score += p1;

    }   

    score
}

#[test]
fn test_get_score(){
    let input = "A Y
B X
C Z";

    assert_eq!(get_score(input), 15)

}


fn main() {
    println!("score: {}", get_score(PAYLOAD));
    println!("score: {}", get_score_part2(PAYLOAD));
}
