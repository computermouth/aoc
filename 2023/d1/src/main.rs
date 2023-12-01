
static PAYLOAD: &'static str = include_str!("../input");

fn get_p1(input: &str) -> u32 {

    let mut rc = 0;
    
    for line in input.lines(){
        let mut first = None;
        let mut last = 0;
        for c in line.chars(){
            match c {
                '0'..='9' => {
                    let u = c.to_digit(10).unwrap();
                    last = u;
                    if first.is_none() {
                        first = Some(u);
                    }

                }
                _ => {}
            }
        }
        println!("f: {} l: {}", first.unwrap() * 10, last);
        rc += first.unwrap() * 10 + last;
    }

    rc
}

#[test]
fn test_p1(){

    let payload = 
    "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    assert_eq!(get_p1(payload), 142);

}

fn get_p2(input: &str) -> u32 {
    let mut rc = 0;

    let words = ["one","two","three","four","five","six","seven","eight","nine"];

    for line in input.lines(){
        let mut first = None;
        let mut last = 0;
        for i in 0..line.len(){
            let strslice = &line[i..];

            let n = strslice.chars().nth(0).unwrap().to_digit(10);
            let s = words.into_iter().position(|x| strslice.starts_with(x));
            println!("n: {:?} s: {:?}", n, s);

            if n.is_some(){ last = n.unwrap(); } else
            if s.is_some(){ last = s.unwrap() as u32 + 1; }
            if first.is_none() {
                if n.is_some(){ first = Some(n.unwrap()); }
                if s.is_some(){ first = Some(s.unwrap() as u32 + 1); }
            }

        }
        println!("f: {} l: {}", first.unwrap() * 10, last);
        rc += first.unwrap() * 10 + last;
    }
    rc
}

#[test]
fn test_p2(){

    let payload = 
    "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    assert_eq!(get_p2(payload), 281);

}

fn main() {
    println!("p1: {}", get_p1(PAYLOAD));
    println!("p2: {}", get_p2(PAYLOAD));
}
