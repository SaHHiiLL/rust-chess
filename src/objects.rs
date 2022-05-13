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
}
