//! lord strike me down for this sin against graydon hoare and the rust foundation
use std::ops::Add;

#[derive(Debug, Clone)]
struct Direction {
    turn_direction: TurnDirection,
    distance: i32,
}
impl Direction {
    pub fn from_string(s: String) -> Self {
        let dir_str = match s.get(0..1) {
            None => panic!("fuck"),
            Some(string) => string,
        };

        let distance: i32 = match s.get(1..) {
            None => panic!("fuck 2"),
            Some(string) => string
        }.parse::<i32>().unwrap();

        let dir = match dir_str {
            "R" => TurnDirection::Right,
            "L" => TurnDirection::Left,
            _ => panic!("fuck 3"),
        };

        Self { turn_direction: dir, distance }
    }

    pub fn from_string_vector(sv: String) -> Vec<Self> {
        sv.split(", ").map(|s| Direction::from_string(s.to_owned())).collect()
    }
}

#[derive(Debug, Clone)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    pub x: i32,
    pub y: i32,
}
impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}
impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let data = String::from("R3, R1, R4, L4, R3, R1, R1, L3, L5, L5, L3, R1, R4, L2, L1, R3, L3, R2, R1, R1, L5, L2, L1, R2, L4, R1, L2, L4, R2, R2, L2, L4, L3, R1, R4, R3, L1, R1, L5, R4, L2, R185, L2, R4, R49, L3, L4, R5, R1, R1, L1, L1, R2, L1, L4, R4, R5, R4, L3, L5, R1, R71, L1, R1, R186, L5, L2, R5, R4, R1, L5, L2, R3, R2, R5, R5, R4, R1, R4, R2, L1, R4, L1, L4, L5, L4, R4, R5, R1, L2, L4, L1, L5, L3, L5, R2, L5, R4, L4, R3, R3, R1, R4, L1, L2, R2, L1, R4, R2, R2, R5, R2, R5, L1, R1, L4, R5, R4, R2, R4, L5, R3, R2, R5, R3, L3, L5, L4, L3, L2, L2, R3, R2, L1, L1, L5, R1, L3, R3, R4, R5, L3, L5, R1, L3, L5, L5, L2, R1, L3, L1, L3, R4, L1, R3, L2, L2, R3, R3, R4, R4, R1, L4, R1, L5");
    let directions = Direction::from_string_vector(data);
    let cardinals = vec!["north", "east", "south", "west"];
    let mut dir_idx: i32 = 0;

    let mut visited: Vec<Vec2> = vec![Vec2::zero()];
    
    let mut vector = Vec2::zero();

    for direction in directions {
        match direction.turn_direction {
            TurnDirection::Left => dir_idx -= 1,
            TurnDirection::Right => dir_idx += 1,
        }

        // hardcoded shite
        if dir_idx == -1 { dir_idx = 3; }
        if dir_idx == 4 { dir_idx = 0; }

        match *cardinals.get::<usize>(dir_idx.try_into().unwrap()).unwrap() {
            "north" => {
                for i in (0..direction.distance.abs()) {
                    vector.y += 1;
                    if visited.contains(&vector) {
                        dbg!(&vector);
                    }
                    visited.push(vector.clone());
                }
            },
            "south" => {
                for i in (0..direction.distance.abs()) {
                    vector.y -= 1;
                    if visited.contains(&vector) {
                        dbg!(&vector);
                    }
                    visited.push(vector.clone());
                }
            },
            "east" => {
                for i in (0..direction.distance.abs()) {
                    vector.x += 1;
                    if visited.contains(&vector) {
                        dbg!(&vector);
                    }
                    visited.push(vector.clone());
                }
            },
            "west" => {
                for i in (0..direction.distance.abs()) {
                    vector.x -= 1;
                    if visited.contains(&vector) {
                        dbg!(&vector);
                    }
                    visited.push(vector.clone());
                }
            },
            _ => panic!("fuck 4")
        }
    }
    
    dbg!(&vector.x, &vector.y);
    println!("{:?}", vector.x.abs() + vector.y.abs());
}
