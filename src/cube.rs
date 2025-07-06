use ratatui::style::Color;
use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum Face {
    Up,
    Down,
    Front,
    Back,
    Right,
    Left,
}

impl Face {
    pub fn opposite_face(self) -> Self {
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

impl From<char> for Face {
    fn from(value: char) -> Self {
        match value {
            'U' => Face::Up,
            'D' => Face::Down,
            'F' => Face::Front,
            'B' => Face::Back,
            'R' => Face::Right,
            'L' => Face::Left,
            _ => panic!(),
        }
    }
}

impl From<Face> for Color {
    fn from(value: Face) -> Self {
        match value {
            Face::Up => Color::Rgb(255, 255, 255),
            Face::Down => Color::Rgb(253, 216, 53),
            Face::Front => Color::Rgb(2, 208, 64),
            Face::Back => Color::Rgb(48, 79, 254),
            Face::Right => Color::Rgb(236, 0, 0),
            Face::Left => Color::Rgb(255, 139, 36),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Rotation {
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

impl From<char> for Rotation {
    fn from(value: char) -> Self {
        match value {
            '\0' => Rotation::Clockwise,
            '\'' => Rotation::CounterClockwise,
            '2' => Rotation::DoubleTurn,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Move {
    pub face: Face,
    pub rotation: Rotation,
}

impl Move {
    pub fn new() -> Self {
        let face: Face = rand::random();
        let rotation: Rotation = rand::random();

        Self { face, rotation }
    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();

        Self {
            face: chars.next().unwrap().into(),
            rotation: chars.next().unwrap_or_default().into(),
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.face, self.rotation)
    }
}

pub struct Cube {
    pub facelets: Vec<Face>,
}

impl Cube {
    pub fn new() -> Self {
        let mut facelets = Vec::new();

        for _ in 0..9 {
            facelets.push(Face::Up);
        }

        for _ in 0..9 {
            facelets.push(Face::Left);
        }

        for _ in 0..9 {
            facelets.push(Face::Front);
        }

        for _ in 0..9 {
            facelets.push(Face::Right);
        }

        for _ in 0..9 {
            facelets.push(Face::Back);
        }

        for _ in 0..9 {
            facelets.push(Face::Down);
        }

        Self { facelets }
    }

    pub fn apply_move(&mut self, r#move: &Move) {
        let c = self.facelets.clone();

        match r#move {
            Move {
                face: Face::Up,
                rotation: Rotation::Clockwise,
            } => {
                self.facelets[0] = c[6];
                self.facelets[1] = c[3];
                self.facelets[2] = c[0];
                self.facelets[3] = c[7];
                self.facelets[5] = c[1];
                self.facelets[6] = c[8];
                self.facelets[7] = c[5];
                self.facelets[8] = c[2];

                self.facelets[9] = c[18];
                self.facelets[10] = c[19];
                self.facelets[11] = c[20];

                self.facelets[18] = c[27];
                self.facelets[19] = c[28];
                self.facelets[20] = c[29];

                self.facelets[27] = c[36];
                self.facelets[28] = c[37];
                self.facelets[29] = c[38];

                self.facelets[36] = c[9];
                self.facelets[37] = c[10];
                self.facelets[38] = c[11];
            }
            Move {
                face: Face::Up,
                rotation: Rotation::CounterClockwise,
            } => {
                self.facelets[0] = c[2];
                self.facelets[1] = c[5];
                self.facelets[2] = c[8];
                self.facelets[3] = c[1];
                self.facelets[5] = c[7];
                self.facelets[6] = c[0];
                self.facelets[7] = c[3];
                self.facelets[8] = c[6];

                self.facelets[9] = c[36];
                self.facelets[10] = c[37];
                self.facelets[11] = c[38];

                self.facelets[18] = c[9];
                self.facelets[19] = c[10];
                self.facelets[20] = c[11];

                self.facelets[27] = c[18];
                self.facelets[28] = c[19];
                self.facelets[29] = c[20];

                self.facelets[36] = c[27];
                self.facelets[37] = c[28];
                self.facelets[38] = c[29];
            }
            Move {
                face: Face::Up,
                rotation: Rotation::DoubleTurn,
            } => {
                self.facelets[0] = c[8];
                self.facelets[1] = c[7];
                self.facelets[2] = c[6];
                self.facelets[3] = c[5];
                self.facelets[5] = c[3];
                self.facelets[6] = c[2];
                self.facelets[7] = c[1];
                self.facelets[8] = c[0];

                self.facelets[9] = c[27];
                self.facelets[10] = c[28];
                self.facelets[11] = c[29];

                self.facelets[18] = c[36];
                self.facelets[19] = c[37];
                self.facelets[20] = c[38];

                self.facelets[27] = c[9];
                self.facelets[28] = c[10];
                self.facelets[29] = c[11];

                self.facelets[36] = c[18];
                self.facelets[37] = c[19];
                self.facelets[38] = c[20];
            }
            Move {
                face: Face::Down,
                rotation: Rotation::Clockwise,
            } => {
                self.facelets[45] = c[51];
                self.facelets[46] = c[48];
                self.facelets[47] = c[45];
                self.facelets[48] = c[52];
                self.facelets[50] = c[46];
                self.facelets[51] = c[53];
                self.facelets[52] = c[50];
                self.facelets[53] = c[47];

                self.facelets[15] = c[42];
                self.facelets[16] = c[43];
                self.facelets[17] = c[44];

                self.facelets[24] = c[15];
                self.facelets[25] = c[16];
                self.facelets[26] = c[17];

                self.facelets[33] = c[24];
                self.facelets[34] = c[25];
                self.facelets[35] = c[26];

                self.facelets[42] = c[33];
                self.facelets[43] = c[34];
                self.facelets[44] = c[35];
            }
            Move {
                face: Face::Down,
                rotation: Rotation::CounterClockwise,
            } => {
                self.facelets[45] = c[47];
                self.facelets[46] = c[50];
                self.facelets[47] = c[53];
                self.facelets[48] = c[46];
                self.facelets[50] = c[52];
                self.facelets[51] = c[45];
                self.facelets[52] = c[48];
                self.facelets[53] = c[51];

                self.facelets[15] = c[24];
                self.facelets[16] = c[25];
                self.facelets[17] = c[26];

                self.facelets[24] = c[33];
                self.facelets[25] = c[34];
                self.facelets[26] = c[35];

                self.facelets[33] = c[42];
                self.facelets[34] = c[43];
                self.facelets[35] = c[44];

                self.facelets[42] = c[15];
                self.facelets[43] = c[16];
                self.facelets[44] = c[17];
            }
            Move {
                face: Face::Down,
                rotation: Rotation::DoubleTurn,
            } => {
                self.facelets[45] = c[53];
                self.facelets[46] = c[52];
                self.facelets[47] = c[51];
                self.facelets[48] = c[50];
                self.facelets[50] = c[48];
                self.facelets[51] = c[47];
                self.facelets[52] = c[46];
                self.facelets[53] = c[45];

                self.facelets[15] = c[33];
                self.facelets[16] = c[34];
                self.facelets[17] = c[35];

                self.facelets[24] = c[42];
                self.facelets[25] = c[43];
                self.facelets[26] = c[44];

                self.facelets[33] = c[15];
                self.facelets[34] = c[16];
                self.facelets[35] = c[17];

                self.facelets[42] = c[24];
                self.facelets[43] = c[25];
                self.facelets[44] = c[26];
            }
            Move {
                face: Face::Front,
                rotation: Rotation::Clockwise,
            } => {
                self.facelets[18] = c[24];
                self.facelets[19] = c[21];
                self.facelets[20] = c[18];
                self.facelets[21] = c[25];
                self.facelets[23] = c[19];
                self.facelets[24] = c[26];
                self.facelets[25] = c[23];
                self.facelets[26] = c[20];

                self.facelets[6] = c[17];
                self.facelets[7] = c[14];
                self.facelets[8] = c[11];

                self.facelets[11] = c[45];
                self.facelets[14] = c[46];
                self.facelets[17] = c[47];

                self.facelets[27] = c[6];
                self.facelets[30] = c[7];
                self.facelets[33] = c[8];

                self.facelets[45] = c[33];
                self.facelets[46] = c[30];
                self.facelets[47] = c[27];
            }
            Move {
                face: Face::Front,
                rotation: Rotation::CounterClockwise,
            } => {
                self.facelets[18] = c[20];
                self.facelets[19] = c[23];
                self.facelets[20] = c[26];
                self.facelets[21] = c[19];
                self.facelets[23] = c[25];
                self.facelets[24] = c[18];
                self.facelets[25] = c[21];
                self.facelets[26] = c[24];

                self.facelets[6] = c[27];
                self.facelets[7] = c[30];
                self.facelets[8] = c[33];

                self.facelets[11] = c[8];
                self.facelets[14] = c[7];
                self.facelets[17] = c[6];

                self.facelets[27] = c[47];
                self.facelets[30] = c[46];
                self.facelets[33] = c[45];

                self.facelets[45] = c[11];
                self.facelets[46] = c[14];
                self.facelets[47] = c[17];
            }
            Move {
                face: Face::Front,
                rotation: Rotation::DoubleTurn,
            } => {
                self.facelets[18] = c[26];
                self.facelets[19] = c[25];
                self.facelets[20] = c[24];
                self.facelets[21] = c[23];
                self.facelets[23] = c[21];
                self.facelets[24] = c[20];
                self.facelets[25] = c[19];
                self.facelets[26] = c[18];

                self.facelets[6] = c[47];
                self.facelets[7] = c[46];
                self.facelets[8] = c[45];

                self.facelets[11] = c[33];
                self.facelets[14] = c[30];
                self.facelets[17] = c[27];

                self.facelets[27] = c[17];
                self.facelets[30] = c[14];
                self.facelets[33] = c[11];

                self.facelets[45] = c[8];
                self.facelets[46] = c[7];
                self.facelets[47] = c[6];
            }
            Move {
                face: Face::Back,
                rotation: Rotation::Clockwise,
            } => {
                self.facelets[36] = c[42];
                self.facelets[37] = c[39];
                self.facelets[38] = c[36];
                self.facelets[39] = c[43];
                self.facelets[41] = c[37];
                self.facelets[42] = c[44];
                self.facelets[43] = c[41];
                self.facelets[44] = c[38];

                self.facelets[0] = c[29];
                self.facelets[1] = c[32];
                self.facelets[2] = c[35];

                self.facelets[9] = c[2];
                self.facelets[12] = c[1];
                self.facelets[15] = c[0];

                self.facelets[29] = c[53];
                self.facelets[32] = c[52];
                self.facelets[35] = c[51];

                self.facelets[51] = c[9];
                self.facelets[52] = c[12];
                self.facelets[53] = c[15];
            }
            Move {
                face: Face::Back,
                rotation: Rotation::CounterClockwise,
            } => {
                self.facelets[36] = c[38];
                self.facelets[37] = c[41];
                self.facelets[38] = c[44];
                self.facelets[39] = c[37];
                self.facelets[41] = c[43];
                self.facelets[42] = c[36];
                self.facelets[43] = c[39];
                self.facelets[44] = c[42];

                self.facelets[0] = c[15];
                self.facelets[1] = c[12];
                self.facelets[2] = c[9];

                self.facelets[9] = c[51];
                self.facelets[12] = c[52];
                self.facelets[15] = c[53];

                self.facelets[29] = c[0];
                self.facelets[32] = c[1];
                self.facelets[35] = c[2];

                self.facelets[51] = c[35];
                self.facelets[52] = c[32];
                self.facelets[53] = c[29];
            }
            Move {
                face: Face::Back,
                rotation: Rotation::DoubleTurn,
            } => {
                self.facelets[36] = c[44];
                self.facelets[37] = c[43];
                self.facelets[38] = c[42];
                self.facelets[39] = c[41];
                self.facelets[41] = c[39];
                self.facelets[42] = c[38];
                self.facelets[43] = c[37];
                self.facelets[44] = c[36];

                self.facelets[0] = c[53];
                self.facelets[1] = c[52];
                self.facelets[2] = c[51];

                self.facelets[9] = c[35];
                self.facelets[12] = c[32];
                self.facelets[15] = c[29];

                self.facelets[29] = c[15];
                self.facelets[32] = c[12];
                self.facelets[35] = c[9];

                self.facelets[51] = c[2];
                self.facelets[52] = c[1];
                self.facelets[53] = c[0];
            }
            Move {
                face: Face::Right,
                rotation: Rotation::Clockwise,
            } => {
                self.facelets[27] = c[33];
                self.facelets[28] = c[30];
                self.facelets[29] = c[27];
                self.facelets[30] = c[34];
                self.facelets[32] = c[28];
                self.facelets[33] = c[35];
                self.facelets[34] = c[32];
                self.facelets[35] = c[29];

                self.facelets[2] = c[20];
                self.facelets[5] = c[23];
                self.facelets[8] = c[26];

                self.facelets[20] = c[47];
                self.facelets[23] = c[50];
                self.facelets[26] = c[53];

                self.facelets[36] = c[8];
                self.facelets[39] = c[5];
                self.facelets[42] = c[2];

                self.facelets[47] = c[42];
                self.facelets[50] = c[39];
                self.facelets[53] = c[36];
            }
            Move {
                face: Face::Right,
                rotation: Rotation::CounterClockwise,
            } => {
                self.facelets[27] = c[29];
                self.facelets[28] = c[32];
                self.facelets[29] = c[35];
                self.facelets[30] = c[28];
                self.facelets[32] = c[34];
                self.facelets[33] = c[27];
                self.facelets[34] = c[30];
                self.facelets[35] = c[33];

                self.facelets[2] = c[42];
                self.facelets[5] = c[39];
                self.facelets[8] = c[36];

                self.facelets[20] = c[2];
                self.facelets[23] = c[5];
                self.facelets[26] = c[8];

                self.facelets[36] = c[53];
                self.facelets[39] = c[50];
                self.facelets[42] = c[47];

                self.facelets[47] = c[20];
                self.facelets[50] = c[23];
                self.facelets[53] = c[26];
            }
            Move {
                face: Face::Right,
                rotation: Rotation::DoubleTurn,
            } => {
                self.facelets[27] = c[35];
                self.facelets[28] = c[34];
                self.facelets[29] = c[33];
                self.facelets[30] = c[32];
                self.facelets[32] = c[30];
                self.facelets[33] = c[29];
                self.facelets[34] = c[28];
                self.facelets[35] = c[27];

                self.facelets[2] = c[47];
                self.facelets[5] = c[50];
                self.facelets[8] = c[53];

                self.facelets[20] = c[42];
                self.facelets[23] = c[39];
                self.facelets[26] = c[36];

                self.facelets[36] = c[26];
                self.facelets[39] = c[23];
                self.facelets[42] = c[20];

                self.facelets[47] = c[2];
                self.facelets[50] = c[5];
                self.facelets[53] = c[8];
            }
            Move {
                face: Face::Left,
                rotation: Rotation::Clockwise,
            } => {
                self.facelets[9] = c[15];
                self.facelets[10] = c[12];
                self.facelets[11] = c[9];
                self.facelets[12] = c[16];
                self.facelets[14] = c[10];
                self.facelets[15] = c[17];
                self.facelets[16] = c[14];
                self.facelets[17] = c[11];

                self.facelets[0] = c[44];
                self.facelets[3] = c[41];
                self.facelets[6] = c[38];

                self.facelets[18] = c[0];
                self.facelets[21] = c[3];
                self.facelets[24] = c[6];

                self.facelets[38] = c[51];
                self.facelets[41] = c[48];
                self.facelets[44] = c[45];

                self.facelets[45] = c[18];
                self.facelets[48] = c[21];
                self.facelets[51] = c[24];
            }
            Move {
                face: Face::Left,
                rotation: Rotation::CounterClockwise,
            } => {
                self.facelets[9] = c[11];
                self.facelets[10] = c[14];
                self.facelets[11] = c[17];
                self.facelets[12] = c[10];
                self.facelets[14] = c[16];
                self.facelets[15] = c[9];
                self.facelets[16] = c[12];
                self.facelets[17] = c[15];

                self.facelets[0] = c[18];
                self.facelets[3] = c[21];
                self.facelets[6] = c[24];

                self.facelets[18] = c[45];
                self.facelets[21] = c[48];
                self.facelets[24] = c[51];

                self.facelets[38] = c[6];
                self.facelets[41] = c[3];
                self.facelets[44] = c[0];

                self.facelets[45] = c[44];
                self.facelets[48] = c[41];
                self.facelets[51] = c[38];
            }
            Move {
                face: Face::Left,
                rotation: Rotation::DoubleTurn,
            } => {
                self.facelets[9] = c[17];
                self.facelets[10] = c[16];
                self.facelets[11] = c[15];
                self.facelets[12] = c[14];
                self.facelets[14] = c[12];
                self.facelets[15] = c[11];
                self.facelets[16] = c[10];
                self.facelets[17] = c[9];

                self.facelets[0] = c[45];
                self.facelets[3] = c[48];
                self.facelets[6] = c[51];

                self.facelets[18] = c[44];
                self.facelets[21] = c[41];
                self.facelets[24] = c[38];

                self.facelets[38] = c[24];
                self.facelets[41] = c[21];
                self.facelets[44] = c[18];

                self.facelets[45] = c[0];
                self.facelets[48] = c[3];
                self.facelets[51] = c[6];
            }
        }
    }
}
