
static PAYLOAD: &'static str = include_str!("../input");

fn get_top_three(s: &str) -> usize {
    let mut all = Vec::new();
    all.push(0);
    let mut current = 0;

    for line in s.lines() {

        match line.parse::<usize>() {
            Ok(n) => current += n,
            Err(_) => {
                let i = all.len() - 1;
                all[i] = current;
                all.push(0);
                current = 0;
            },
        }
    }
    // last number has no blank line
    all.push(current);

    all.sort();
    let size = all.len();

    all[size -1] + all[size -2] + all[size - 3]
}

#[test]
fn test_get_top_three(){
    let t = 
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    assert_eq!(get_top_three(t), 45000);

}

fn get_highest(s: &str) -> usize {

    let mut current = 0;
    let mut max = 0;

    for line in s.lines() {

        match line.parse::<usize>() {
            Ok(n) => current += n,
            Err(_) => {
                if current > max { max = current;}
                current = 0;
            },
        }
    }

    max
}

#[test]
fn test_get_highest(){
    let t = 
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    assert_eq!(get_highest(t), 24000);

}

fn main() {
    println!("highest: {}", get_highest(PAYLOAD));
    println!("top 3: {}", get_top_three(PAYLOAD));
}
