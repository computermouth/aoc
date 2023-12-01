use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

static TEST1_PAYLOAD: &'static str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

// 2Hz
const RENDER_STEP: f32 = 0.016 * 60.;

#[derive (Debug)]
struct Entity {
    item_calories: Vec<usize>,
    item_total: usize,
    draw_time_remaining: f32
}

impl Entity {
    fn new() -> Entity {
        Entity{ item_calories: Vec::new(), item_total: 0, draw_time_remaining: 0.}
    }
}

#[derive (Debug)]
enum Stage {
    TEST1,
    PART1,
    TEST2,
    PART2,
}

struct ParsedData {
    test1ents: Vec<Entity>,
    part1ents: Vec<Entity>,
}

fn parse_part1(s: &str) -> Vec<Entity> {

    let mut entities = Vec::new();
    let mut e = Entity::new();
    let mut items = Vec::new();
    for line in s.lines() {
        match line.parse::<usize>(){
            Ok(n) => {
                items.push(n);
                e.item_total += n;
                e.draw_time_remaining += RENDER_STEP;
            },
            Err(_) => {
                e.item_calories = items;
                entities.push(e);
                e = Entity::new();
                items = Vec::new();
            }
        }
    }

    entities
}

fn parse_data() -> ParsedData {
    let p = ParsedData {
        test1ents: parse_part1(TEST1_PAYLOAD),
        part1ents: parse_part1(TEST1_PAYLOAD),
    };

    p
}

fn draw_part1_test(delta: f32, entities: &mut Vec<Entity>) -> Stage {

    for (i, v) in entities.into_iter().enumerate() {
        if v.draw_time_remaining < 0. {
            for (j, c) in v.item_calories.iter().enumerate() {
                draw_cube(WORLD_CENTER+ vec3(i as f32 * 4., j as f32 * 2., 0.), vec3(2., *c as f32 / 1000., 2.), None, GREEN);
                draw_cube_wires(WORLD_CENTER+ vec3(i as f32 * 4., j as f32 * 2., 0.), vec3(2., 2., 2.), BLACK);
            }
        } else {
            v.draw_time_remaining -= delta * 10.;
            println!("draw: {}\n delta: {}", v.draw_time_remaining, delta);
            return Stage::TEST1;
        }
    }

    Stage::TEST1
}

const WORLD_CENTER : Vec3 = Vec3{x: 100., y: 100., z: 100.};

fn update(delta: f32, stage: &mut Stage, data: &mut ParsedData){

    *stage = match stage {
        Stage::TEST1 => { draw_part1_test(delta, &mut data.test1ents) },
        _ => { Stage::PART1 },
        // PART1 => {},
        // TEST2 => {},
        // PART2 => {}
    };

}

#[macroquad::main(conf)]
async fn main() {

    let mut stage = Stage::TEST1;

    let mut data = parse_data();

    loop {

        let delta = get_frame_time() % 0.16;

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(DARKGRAY);

        set_camera(&Camera3D{
            position: WORLD_CENTER + vec3(1., 20., 20.),
            up: vec3(0., 1., 0.),
            target: WORLD_CENTER,
            ..Default::default()
        });

        draw_plane(WORLD_CENTER - vec3(0.,1.,0.), vec2(20., 20.), None, GRAY);

        update(delta, &mut stage, &mut data);

        set_default_camera();

        draw_text("--day 1--", 10.0, 20.0, 30.0, BLACK);

        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            48.0 + 18.0,
            30.0,
            BLACK,
        );

        draw_text(
            format!("Stage: {:?}", stage).as_str(),
            10.0,
            48.0 + 42.0,
            30.0,
            BLACK,
        );

        next_frame().await
    }

}
