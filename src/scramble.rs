use crate::cube::{Face, Move, Rotation};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt::Display;

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

impl Distribution<Rotation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rotation {
        match rng.gen_range(0..=2) {
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
