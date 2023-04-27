use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt::Display;

#[derive(PartialEq, Clone)]
enum Face {
    Up,
    Down,
    Front,
    Back,
    Right,
    Left,
}

impl Face {
    fn opposite_face(&self) -> Self {
        match self {
            Face::Up => Face::Down,
            Face::Down => Face::Up,
            Face::Front => Face::Back,
            Face::Back => Face::Front,
            Face::Right => Face::Left,
            Face::Left => Face::Right,
        }
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Face::Up => "U",
                Face::Down => "D",
                Face::Front => "F",
                Face::Back => "B",
                Face::Right => "R",
                Face::Left => "L",
            }
        )
    }
}

impl Distribution<Face> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Face {
        match rng.gen_range(0..=5) {
            0 => Face::Up,
            1 => Face::Down,
            2 => Face::Front,
            3 => Face::Back,
            4 => Face::Right,
            _ => Face::Left,
        }
    }
}

#[derive(PartialEq, Clone)]
enum Rotation {
    Clockwise,
    CounterClockwise,
    DoubleTurn,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rotation::Clockwise => "",
                Rotation::CounterClockwise => "'",
                Rotation::DoubleTurn => "2",
            }
        )
    }
}

impl Distribution<Rotation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation {
        match rng.gen_range(0..=2) {
            0 => Rotation::Clockwise,
            1 => Rotation::CounterClockwise,
            _ => Rotation::DoubleTurn,
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Move {
    face: Face,
    rotation: Rotation,
}

impl Move {
    fn new() -> Self {
        let face: Face = rand::random();
        let rotation: Rotation = rand::random();

        Self { face, rotation }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.face, self.rotation)
    }
}

pub struct Scramble {
    pub moves: Vec<Move>,
}

impl Scramble {
    pub fn new(scramble_length: u8) -> Self {
        let mut moves = vec![Move::new()];

        let mut previous_move = moves[0].clone();
        let mut was_opposite = false;

        for _ in 1..=scramble_length {
            moves.push(loop {
                let r#move = Move::new();

                if r#move.face != previous_move.face
                    && !(r#move.face == previous_move.face.opposite_face() && was_opposite)
                {
                    was_opposite = r#move.face == previous_move.face.opposite_face();
                    previous_move = r#move.clone();
                    break r#move;
                }
            })
        }

        Self { moves }
    }
}

impl Display for Scramble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.moves
                .iter()
                .map(|r#move| r#move.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
