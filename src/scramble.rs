use crate::cube::{Face, Move, Rotation};
use rand::distr::{Distribution, StandardUniform};
use rand::Rng;
use std::fmt::Display;

impl Distribution<Face> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Face {
        match rng.random_range(0..=5) {
            0 => Face::Up,
            1 => Face::Down,
            2 => Face::Front,
            3 => Face::Back,
            4 => Face::Right,
            _ => Face::Left,
        }
    }
}

impl Distribution<Rotation> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation {
        match rng.random_range(0..=2) {
            0 => Rotation::Clockwise,
            1 => Rotation::CounterClockwise,
            _ => Rotation::DoubleTurn,
        }
    }
}

#[derive(Clone)]
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
            });
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
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl From<&str> for Scramble {
    fn from(value: &str) -> Self {
        let mut moves = Vec::new();

        for r#move in value.split_whitespace() {
            moves.push(r#move.into());
        }

        Self { moves }
    }
}
