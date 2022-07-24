pub struct Board {
    pub x: u8,
    pub y: u8,
}

pub struct POS {
    pub x: u8,
    pub y: u8,
}

pub struct Piece {
    pub pos: POS,
    pub colour: COLOUR,
    pub piece_type: TYPE,
}
pub enum COLOUR {
    BLACK,
    WHITE,
}
pub enum TYPE {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
}

impl Board {
    pub const fn new() -> Board {
        Board { x: 8, y: 8 }
    }

    //MAKe this a list of pieces
    pub fn draw_the_board(&self, pieces: Vec<Piece>) {
        let x = self.x;
        let y = self.y;

        fn is_pos_point(i: u8, j: u8, pos: &POS) -> bool {
            pos.x == i && pos.y == j
        }

        fn is_pos_point_in_vec(pos: &POS, vec: &Vec<Piece>) -> bool {
            for piece in vec {
                if is_pos_point(pos.x, pos.y, &piece.pos) {
                    Board::print_piece(&piece);
                    return true;
                }
            }
            false
        }

        for i in 1..=x {
            for j in 1..=y {
                //TODO
                if !is_pos_point_in_vec(&POS { x: i, y: j }, &pieces) {
                    print!("{}        ", "*");            
                }
            }
            println!();
        }
    }

    fn print_piece(piece: &Piece){
        let mut s: String = match piece.colour {
            COLOUR::BLACK => "B.".to_string(),
            COLOUR::WHITE => "W.".to_string(),
        };

        s.push_str(match piece.piece_type {
            TYPE::KING => "K.",
            TYPE::QUEEN => "Q.",
            TYPE::ROOK => "R.",
            TYPE::BISHOP =>"B.",
            TYPE::KNIGHT =>"N.",
            TYPE::PAWN => "P.",
        });

        print!("{}     ", s);
    }
}

impl Piece {
    pub const fn new(pos: POS, colour: COLOUR, piece_type: TYPE) -> Piece {
        Piece {
            pos,
            colour,
            piece_type,
        }
    }

    pub const fn default_pieces() -> Vec<Self> {
        vec![
            Piece {
                pos: POS { x: 1, y: 1 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::ROOK,
            },
            Piece {
                pos: POS { x: 1, y: 2 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::KNIGHT,
            },
            Piece{
                pos: POS { x: 1, y: 3 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::BISHOP,
            }, 
            Piece {
                pos: POS { x: 1, y: 4 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::QUEEN,
            },
            Piece {
                pos: POS { x: 1, y: 5 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::KING,
            },
            Piece {
                pos: POS { x: 1, y: 6 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::BISHOP,

            },
            Piece {
                pos: POS { x: 1, y: 7 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::KNIGHT,
            },
            Piece {
                pos: POS { x: 1, y: 8 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::ROOK,
            },
            Piece {
                pos: POS { x: 2, y: 1 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 2, y: 2 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::PAWN,
            }, 
            Piece {
                pos: POS { x: 2, y: 3 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 2, y: 4 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 2, y: 5 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 2, y: 6 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 2, y: 7 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 2, y: 8 },
                colour: COLOUR::WHITE,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 7, y: 1 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::PAWN, 
            },
            Piece {
                pos: POS { x: 7, y: 2 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 7, y: 3 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 7, y: 4 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 7, y: 5 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 7, y: 6 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 7, y: 7 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 7, y: 8 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::PAWN,
            },
            Piece {
                pos: POS { x: 8, y: 1 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::ROOK,
            },
            Piece {
                pos: POS { x: 8, y: 2 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::KNIGHT,
            },
            Piece {
                pos: POS { x: 8, y: 3 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::BISHOP,
            },
            Piece {
                pos: POS { x: 8, y: 4 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::QUEEN,
            },
            Piece {
                pos: POS { x: 8, y: 5 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::KING,
            },
            Piece {
                pos: POS { x: 8, y: 6 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::BISHOP,
            },
            Piece {
                pos: POS { x: 8, y: 7 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::KNIGHT,
            },
            Piece {
                pos: POS { x: 8, y: 8 },
                colour: COLOUR::BLACK,
                piece_type: TYPE::ROOK,
            },
        ]
    }

    pub fn equal (&self, piece: Piece) -> bool {
        self.pos.x == piece.pos.x && self.pos.y == piece.pos.y
    }
}
