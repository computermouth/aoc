
static PAYLOAD: &'static str = include_str!("../input");

fn get_p1(input: &str) -> u32 {

    let mut rc = 0;
    
    for line in input.lines(){
        // let (game)
        let spaceless = line.replace(' ', "");

        let game_split: Vec<&str> = spaceless.split("Game").collect();
        let num_split: Vec<&str> = game_split[1].split(":").collect();
        let game_num = num_split[0].parse::<u32>().unwrap();

        let game_results_split: Vec<&str> = num_split[1].split(";").collect();

        // r, g, b
        let mut games: [u32; 3] = [0, 0, 0];

        for res in game_results_split {
            let sets: Vec<&str> = res.split(',').collect();
            for set in sets {
                if set.ends_with("red") {
                    let t = set.replace("red", "");
                    let shown = t.parse::<u32>().unwrap();
                    if shown > games[0] {
                        games[0] = shown;
                    }
                }
                if set.ends_with("green") {
                    let t = set.replace("green", "");
                    let shown = t.parse::<u32>().unwrap();
                    if shown > games[1] {
                        games[1] = shown;
                    }
                }
                if set.ends_with("blue") {
                    let t = set.replace("blue", "");
                    let shown = t.parse::<u32>().unwrap();
                    if shown > games[2] {
                        games[2] = shown;
                    }
                }
            } 
        }

        let mut good = false;
        if games[0] <= 12 && games[1] <= 13 && games[2] <=14 {
            rc += game_num;
            good = true;
        }

        // .into_iter().map(|x| x.split('"')).collect();
        println!("{:?} -- {:?} -- {:?}", game_num, games, good);
    }

    rc
}

fn get_p2(input: &str) -> u32 {

    let mut rc = 0;
    
    for line in input.lines(){
        // let (game)
        let spaceless = line.replace(' ', "");

        let game_split: Vec<&str> = spaceless.split("Game").collect();
        let num_split: Vec<&str> = game_split[1].split(":").collect();
        let game_num = num_split[0].parse::<u32>().unwrap();

        let game_results_split: Vec<&str> = num_split[1].split(";").collect();

        // r, g, b
        let mut games: [u32; 3] = [0, 0, 0];

        for res in game_results_split {
            let sets: Vec<&str> = res.split(',').collect();
            for set in sets {
                if set.ends_with("red") {
                    let t = set.replace("red", "");
                    let shown = t.parse::<u32>().unwrap();
                    if shown > games[0] {
                        games[0] = shown;
                    }
                }
                if set.ends_with("green") {
                    let t = set.replace("green", "");
                    let shown = t.parse::<u32>().unwrap();
                    if shown > games[1] {
                        games[1] = shown;
                    }
                }
                if set.ends_with("blue") {
                    let t = set.replace("blue", "");
                    let shown = t.parse::<u32>().unwrap();
                    if shown > games[2] {
                        games[2] = shown;
                    }
                }
            } 
        }

        rc += games[0] * games[1] * games[2];

        // // .into_iter().map(|x| x.split('"')).collect();
        // println!("{:?} -- {:?} -- {:?}", game_num, games);
    }

    rc
}

#[test]
fn test_p1(){

    let payload = 
r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    assert_eq!(get_p1(payload), 8);

}

#[test]
fn test_p2(){

    let payload = 
r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    assert_eq!(get_p2(payload), 2286);

}

fn main() {
    println!("p1: {}", get_p1(PAYLOAD));
    println!("p2: {}", get_p2(PAYLOAD));
}
