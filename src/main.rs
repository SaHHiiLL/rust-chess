mod objects;
pub use objects::*;
fn main() {
    println!("Hello, world!");
    let b = Board::new(); 
    
    let pieces: Vec<Piece> = vec![
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
    ];
    b.draw_the_board(pieces); 

}

