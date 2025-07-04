use iced::{
    Alignment, Task as Command, Element, Length, color
};
// use iced::theme::{self, Theme};
use iced::widget::{
    button, column, container, row, text, Column, Row
};

mod modal;
mod custom_theme;


pub fn main() -> iced::Result {
    iced::application(Chess::title, Chess::update, Chess::view)
    .run_with(Chess::new)
}


const CHESS_LEHGT: usize = 8;

#[derive(Clone, PartialEq, Copy, Debug)]
enum ChessPiece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Player {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PieceState {
    owner: Player,
    piece: ChessPiece,
}

#[derive(Clone, Copy)]
struct TileState {
    piece_state: Option<PieceState>,
    high_light: bool,
}


#[derive(Debug, Clone)]
struct Point {
    h: usize,
    w: usize,
}

type Board = Vec<Vec<TileState>>;

struct Chess {
    board: Board,
    turn: Player,
    choose: Option<(Point, PieceState)>,
    en_passant: Option<Point>,
    castling: Castling,
    promotion_popup: bool,
    game_over: bool,
}

struct Castling {
    w_r_rook: bool,
    w_l_rook: bool,
    w_king: bool,
    b_r_rook: bool,
    b_l_rook: bool,
    b_king: bool,
}

#[derive(Debug, Clone)]
enum Message {
    MoveAble(Point, Option<PieceState>),
    Move(Point),
    Promotion(PieceState),
    Reset,
}

fn init_board() -> Board {
    let mut board: Board = vec![vec![TileState {piece_state: None, high_light: false}; CHESS_LEHGT]; CHESS_LEHGT];

    // ♜♞♝♛♚♝♞♜
    let chess_seq: [ChessPiece; CHESS_LEHGT] = [
                        ChessPiece::Rook,
                        ChessPiece::Knight,
                        ChessPiece::Bishop,
                        ChessPiece::Queen,
                        ChessPiece::King,
                        ChessPiece::Bishop,
                        ChessPiece::Knight,
                        ChessPiece::Rook,
                        ];

    for i in 0..CHESS_LEHGT {
        board[0][i] = TileState {piece_state: Some( PieceState {owner: Player::Black , piece: chess_seq[i].clone() }), high_light: false};
        board[1][i] = TileState {piece_state: Some( PieceState {owner: Player::Black , piece: ChessPiece::Pawn }), high_light: false};

        
        board[6][i] = TileState {piece_state: Some( PieceState {owner: Player::White , piece: ChessPiece::Pawn }), high_light: false};
        board[7][i] = TileState {piece_state: Some( PieceState {owner: Player::White , piece: chess_seq[i].clone() }), high_light: false};
    }

    board
}

fn view_tile(tile: &TileState, h: usize, w: usize) -> Element<Message>  {
    let b;
    let piece_color;

    if let Some(x) = &tile.piece_state {
        match x.owner {
            Player::White => { piece_color = color!(0xffffff); },
            Player::Black => { piece_color = color!(0x000000); },
        };

        let piece = match x.piece {
            ChessPiece::Bishop  => { text("♝") },
            ChessPiece::King    => { text("♚") },
            ChessPiece::Pawn    => { text("♙") },
            ChessPiece::Queen   => { text("♛") },
            ChessPiece::Rook    => { text("♜") },
            ChessPiece::Knight  => { text("♞") },
        }
        .size(50.0)
        .color(piece_color)
        .shaping(text::Shaping::Advanced)
        .center();
        
        if tile.high_light == true {
            b = button(piece).on_press(Message::Move(Point{ h, w }));
        } else {
            b = button(piece).on_press(Message::MoveAble(Point{ h, w }, Some(*x)));
        }
        
    } else {

        if tile.high_light == true {
            b = button(" ").on_press(Message::Move(Point{ h, w }));
        } else {
            b = button(" ");
        }
    }
    
    
    let butten_style = match tile.high_light {
        true => { custom_theme::ChessStyle::hightlighted_button_wrapper },
        false => {
            match (h, w) {
                (h, w) if (h % 2) ^ (w % 2) == 0 => {custom_theme::ChessStyle::bright_button_wrapper},
                (_,_) => {custom_theme::ChessStyle::dark_button_wrapper},
            }
        }
    };

    b.height(Length::Fixed(80.0)).width(Length::Fixed(80.0))
        .style(
            move |_,state| butten_style(state))
        .into()
}

fn move_able_rook(board: &Board, p: &Point) -> Vec<Point> {
    let mut high_light: Vec<Point> = Vec::new();

    let w = p.w;
    let h = p.h;

    let piece = board[h][w].piece_state.clone().unwrap();

    let mut dw = w as i32 - 1;
    while dw >= 0{
        if let Some(x) = board[h][dw as usize].piece_state {
            if x.owner != piece.owner {
                high_light.push(Point {h, w: dw as usize} );
            }
            break;
        }
        high_light.push(Point {h: h as usize, w: dw as usize} );
        dw -= 1;
    }

    let mut dw = w as i32 + 1;
    while dw < CHESS_LEHGT as i32 {
        if let Some(x) = board[h][dw as usize].piece_state {
            if x.owner != piece.owner {
                high_light.push(Point {h, w: dw as usize} );
            }
            break;
        }
        high_light.push(Point {h, w: dw as usize} );
        dw += 1;
    }

    let mut dh = h as i32 + 1;
    while dh < CHESS_LEHGT as i32 {
        if let Some(x) = board[dh as usize][w].piece_state {
            if x.owner != piece.owner {
                high_light.push(Point {h : dh as usize, w });
            }
            break;
        }
        high_light.push(Point {h : dh as usize, w });
        dh += 1;   
    }

    let mut dh = h as i32 - 1;
    while dh >= 0{
        if let Some(x) = board[dh as usize][w].piece_state {
            if x.owner != piece.owner {
                high_light.push(Point {h : dh as usize, w });
            }
            break;
        }
        high_light.push(Point {h : dh as usize, w });
        dh -= 1;   
    }


    high_light
}

fn move_able_bishop(board: &Board, p: &Point) -> Vec<Point> {
    let mut high_light: Vec<Point> = Vec::new();

    let w = p.w;
    let h = p.h;

    let piece = board[h][w].piece_state.clone().unwrap();

    let mut dh = h as i32 - 1;
    let mut dw = w as i32 - 1;
    while dh >= 0 && dw >= 0 {
        if let Some(x) = board[dh as usize][dw as usize].piece_state {
            if x.owner != piece.owner {
                high_light.push(Point {h : dh as usize, w: dw as usize });
            }
            break;
        }
        high_light.push(Point {h : dh as usize, w: dw as usize });

        dh -= 1;
        dw -= 1;
    }

    let mut dh = h as i32 + 1;
    let mut dw = w as i32 - 1;
    while dh < CHESS_LEHGT as i32 && dw >= 0 {
        if let Some(x) = board[dh as usize][dw as usize].piece_state {
            if x.owner != piece.owner {
                high_light.push(Point {h : dh as usize, w: dw as usize });
            }
            break;
        }
        high_light.push(Point {h : dh as usize, w: dw as usize });

        dh += 1;
        dw -= 1;
    }

    let mut dh = h as i32 - 1;
    let mut dw = w as i32 + 1;
    while dh >= 0 && dw < CHESS_LEHGT as i32 {
        if let Some(x) = board[dh as usize][dw as usize].piece_state {
            if x.owner != piece.owner {
                high_light.push(Point {h : dh as usize, w: dw as usize });
            }
            break;
        }
        high_light.push(Point {h : dh as usize, w: dw as usize });

        dh -= 1;
        dw += 1;
    }

    let mut dh = h as i32 + 1;
    let mut dw = w as i32 + 1;
    while dh < CHESS_LEHGT as i32 && dw < CHESS_LEHGT as i32 {
        if let Some(x) = board[dh as usize][dw as usize].piece_state {
            if x.owner != piece.owner {
                high_light.push(Point {h : dh as usize, w: dw as usize });
            }
            break;
        }
        high_light.push(Point {h : dh as usize, w: dw as usize });

        dh += 1;
        dw += 1;
    }


    high_light
}

fn move_able_king(board: &Board, p: &Point) -> Vec<Point> {
    let mut high_light: Vec<Point> = Vec::new();

    let w = p.w;
    let h = p.h;

    let piece = board[h][w].piece_state.clone().unwrap();

    for dh in -1..=1 {
        for dw in -1..=1 {
            if ((h as i32 + dh >= 0) && (h as i32 + dh < CHESS_LEHGT as i32)) && ((w as i32 + dw >= 0) && (w as i32 + dw < CHESS_LEHGT as i32)) {
                if dh == 0 && dw == 0 { continue; }

                if let Some(x) = board[(h as i32 + dh) as usize][(w as i32 + dw) as usize].piece_state {
                    if x.owner != piece.owner {
                        high_light.push(Point { h: (h as i32 + dh) as usize, w : (w as i32 + dw) as usize});
                    }
                } else {
                    high_light.push(Point { h: (h as i32 + dh) as usize, w : (w as i32 + dw) as usize});
                }                
            }
        }    
    }

    high_light
}

fn move_able_knight(board: &Board, p: &Point) -> Vec<Point> {
    let mut high_light: Vec<Point> = Vec::new();

    let w = p.w as i32;
    let h = p.h as i32;

    let piece = board[h as usize][w as usize].piece_state.clone().unwrap();
    
    let list = [(2, 1), 
                                 (2, -1),
                                 (-2, 1),
                                 (-2, -1),
                                 (1, 2),
                                 (1, -2),
                                 (-1, 2),
                                 (-1, -2),
                                 ];

    for (dh, dw) in list {
        if h + dh >= 0 && h + dh < CHESS_LEHGT as i32 && w + dw >= 0 && w + dw < CHESS_LEHGT as i32 {
            
            if let Some(x) = board[(h + dh) as usize][(w + dw) as usize].piece_state {
                if x.owner != piece.owner {
                    high_light.push(Point { h: (h + dh) as usize, w : (w + dw) as usize});
                }
            } else {
                high_light.push(Point { h: (h + dh) as usize, w : (w + dw) as usize});
            }
        }
    }


    high_light
}

fn move_able_pawn(board: &Board, p: &Point, en_passant: &Option<Point>) -> Vec<Point> {
    let mut high_light: Vec<Point> = Vec::new();

    let h = p.h;
    let w = p.w;

    let piece = board[h][w].piece_state.clone().unwrap();
    match piece.owner {
        Player::Black => {
            if h == 1 {
                let mut dh = 1;
                while dh <= 2 {
                    if board[h + dh][w].piece_state == None {
                        high_light.push(Point { h: h + dh, w });
                    } else {
                        break;
                    }
                    dh += 1;
                }
            } else {
                if board[h + 1][w].piece_state == None {
                    high_light.push(Point { h: h + 1, w });
                }

            }
            // attck
            if w + 1 < CHESS_LEHGT {
                if let Some(x) = board[h + 1][w + 1].piece_state {
                    if x.owner != piece.owner {
                        high_light.push(Point { h: h + 1, w : w + 1});
                    }
                }
                if let Some(x) = en_passant {
                    if (x.h == h + 1) && (x.w == w + 1) {
                        high_light.push(Point { h: h + 1, w : w + 1});
                    }
                }
            }

            if w as i32 - 1 >= 0 {
                if let Some(x) = board[h + 1][w - 1].piece_state {
                    if x.owner != piece.owner {
                        high_light.push(Point { h: h + 1, w : w - 1});
                    }
                }
                if let Some(x) = en_passant {
                    if (x.h == h + 1) && (x.w == w - 1) {
                        high_light.push(Point { h: h + 1, w : w - 1});
                    }
                }
            }
        },
        Player::White => {
            if h == 6 {
                let mut dh = 1;
                while dh <= 2 {
                    if board[h - dh][w].piece_state == None {
                        high_light.push(Point { h: h - dh, w });
                    } else {
                        break;
                    }
                    dh += 1;
                }
            } else {
                if board[h - 1][w].piece_state == None {
                    high_light.push(Point { h: h - 1, w });
                }

            }
            // attck
            if w + 1 < CHESS_LEHGT {
                if let Some(x) = board[h - 1][w + 1].piece_state {
                    if x.owner != piece.owner {
                        high_light.push(Point { h: h - 1, w : w + 1});
                    }
                }
                if let Some(x) = en_passant {
                    if (x.h == h - 1) && (x.w == w + 1) {
                        high_light.push(Point { h: h - 1, w : w + 1});
                    }
                }
            }

            if w as i32 - 1 >= 0 {
                if let Some(x) = board[h - 1][w - 1].piece_state {
                    if x.owner != piece.owner {
                        high_light.push(Point { h: h - 1, w : w - 1});
                    }
                }
                if let Some(x) = en_passant {
                    if (x.h == h - 1) && (x.w == w - 1) {
                        high_light.push(Point { h: h - 1, w : w - 1});
                    }
                }
            }
        },
    }

    high_light
}

fn move_able_queen(board: &Board, p: &Point) -> Vec<Point> {
    let mut high_light: Vec<Point> = Vec::new();

    high_light.extend(move_able_bishop(&board, &p));
    high_light.extend(move_able_rook(&board, &p));

    high_light
}

fn update_high_light(board: &Board, light: Vec<Point>, value: bool) -> Board {
    let mut cloned_board: Board = board.iter()
        .map(|inner_vector| inner_vector.clone())
        .collect();

    for p in light {
        cloned_board[p.h][p.w].high_light = value;
    }
    cloned_board
}



impl Chess {

    fn new() -> (Self, Command<Message>) {
        let board = init_board();

        (
            Self {
                board,
                turn: Player::White,
                choose: None,
                
                en_passant: None,
                castling: Castling { w_r_rook:false, b_king: false,b_l_rook:false, b_r_rook:false,w_king:false,w_l_rook:false},
                promotion_popup: false,
                game_over: false,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("chess - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MoveAble(p, piece_state) => {
                
                if let Some((point, piece_state)) = &self.choose {

                    let light = match piece_state.piece {
                        ChessPiece::Rook    => { move_able_rook(&self.board, &point) },
                        ChessPiece::Bishop  => { move_able_bishop(&self.board, &point) },
                        ChessPiece::King    => { move_able_king(&self.board, &point) },
                        ChessPiece::Knight  => { move_able_knight(&self.board, &point) },
                        ChessPiece::Queen   => { move_able_queen(&self.board, &point) },

                        ChessPiece::Pawn    => { move_able_pawn(&self.board, &point, &self.en_passant) },
                    };
                
                    self.board = update_high_light(&self.board, light, false);
                }

                if let Some(x) = piece_state {
                    if x.owner == self.turn {
                        
                        let light = match x.piece {
                            ChessPiece::Rook    => { move_able_rook(&self.board, &p) },
                            ChessPiece::Bishop  => { move_able_bishop(&self.board, &p) },
                            ChessPiece::King    => { move_able_king(&self.board, &p) },
                            ChessPiece::Knight  => { move_able_knight(&self.board, &p) },
                            ChessPiece::Queen   => { move_able_queen(&self.board, &p) },
    
                            ChessPiece::Pawn    => { move_able_pawn(&self.board, &p, &self.en_passant) },
                        };
                        self.board = update_high_light(&self.board, light, true);
                        self.choose = Some((p, x));
                    }
                }
                
                Command::none()
            },
            Message::Move(p) => {
                
                if let Some((point, piece_state)) = self.choose.clone() {
                    // en passant kill
                    if piece_state.piece == ChessPiece::Pawn {
                        if let Some(x) = &self.en_passant {
                            if x.h == p.h && x.w == p.w {
                                match piece_state.owner {
                                    Player::White => { self.board[x.h + 1][x.w] = TileState {piece_state: None, high_light: false}; },
                                    Player::Black => { self.board[x.h - 1][x.w] = TileState {piece_state: None, high_light: false}; },
                                }   
                            }
                        }
                    }
                    self.en_passant = None;
                    
                    // en passant able
                    if piece_state.piece == ChessPiece::Pawn {
                        if ((p.h as i32 - point.h as i32).abs() == 2) && (p.w - point.w == 0) {
                            if piece_state.owner == Player::White && p.h == 4 {
                                self.en_passant = Some(Point {h: p.h+1, w: p.w});
                            } else if piece_state.owner == Player::Black && p.h == 3 {
                                self.en_passant = Some(Point {h: p.h-1, w: p.w});
                            }
                        }                    
                    }

                    if let Some(x) = self.board[p.h][p.w].piece_state {
                        if x.piece == ChessPiece::King {
                            self.game_over = true;
                        }
                    }

                    // move
                    let light = match piece_state.piece {
                        ChessPiece::Rook    => { move_able_rook(&self.board, &point) },
                        ChessPiece::Bishop  => { move_able_bishop(&self.board, &point) },
                        ChessPiece::King    => { move_able_king(&self.board, &point) },
                        ChessPiece::Knight  => { move_able_knight(&self.board, &point) },
                        ChessPiece::Queen   => { move_able_queen(&self.board, &point) },

                        ChessPiece::Pawn    => { move_able_pawn(&self.board, &point, &self.en_passant) },
                    };
                    self.board = update_high_light(&self.board, light, false);

                    self.board[p.h][p.w] = self.board[point.h][point.w];
                    self.board[point.h][point.w] = TileState {piece_state: None, high_light: false};
                    self.choose = None;
                    

                    // Promotion
                    if (piece_state.piece == ChessPiece::Pawn) && (((piece_state.owner == Player::White) && (p.h == 0)) || ((piece_state.owner == Player::Black) && (p.h == 7))) {
                        self.promotion_popup = true;
                    }
                    
                    
                    self.turn = match self.turn {
                        Player::White => { Player::Black },
                        Player::Black => { Player::White }
                    }
                }
                


                Command::none()
            },
            Message::Promotion(piece_state) => {
                match piece_state.owner {
                    Player::White => {
                        for i in 0..CHESS_LEHGT {
                            if let Some(x) = self.board[0][i].piece_state {
                                if x.piece == ChessPiece::Pawn {
                                    self.board[0][i].piece_state = Some(piece_state);
                                    break;
                                }
                            }
                        }
                    },
                    Player::Black => {
                        for i in 0..CHESS_LEHGT {
                            if let Some(x) = self.board[7][i].piece_state {
                                if x.piece == ChessPiece::Pawn {
                                    self.board[7][i].piece_state = Some(piece_state);
                                    break;
                                }
                            }
                        }
                    },
                }


                self.promotion_popup = false;
                Command::none()
            },
            Message::Reset => {
                let board = init_board();

                self.board = board;
                self.turn = Player::White;
                self.choose = None;

                self.promotion_popup = false;                
                self.en_passant = None;
                self.castling = Castling { w_r_rook:false, b_king: false,b_l_rook:false, b_r_rook:false,w_king:false,w_l_rook:false};
                self.game_over = false;
                Command::none()     
            },
        }
    }

    fn view(&self) -> Element<Message> {
        

        let board = container((0..CHESS_LEHGT).into_iter().fold(Column::new() ,|c, i|
                c.push(Element::from(
                    (0..CHESS_LEHGT).into_iter().fold(Row::new(),|c, j|
                        c.push(
                            view_tile(&self.board[i][j], i, j)
                        )
                    )
                ))
            ));
        
        let content = container(
                column!(
                    button("reset").on_press(Message::Reset).padding(5),
                    board,
                ).align_x(Alignment::Center)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill);
            


        if self.promotion_popup {
            
            let (text_color, owner) = match self.turn {
                Player::White => (color!(0x000000), Player::Black),
                Player::Black => (color!(0xffffff), Player::White),
            };

            let promotion_modal = container(
                column![
                    text("promotion").size(20.0),
                    row![
                        button(text("♛").size(50.0).color(text_color).shaping(text::Shaping::Advanced)).on_press(Message::Promotion(PieceState {owner, piece: ChessPiece::Queen}))
                            .style( |_,state| { custom_theme::ChessStyle::bright_button_wrapper(state) } ),
                        
                        button(text("♝").size(50.0).color(text_color).shaping(text::Shaping::Advanced)).on_press(Message::Promotion(PieceState {owner, piece: ChessPiece::Bishop}))
                            .style( |_,state| { custom_theme::ChessStyle::dark_button_wrapper(state) } ),

                        button(text("♞").size(50.0).color(text_color).shaping(text::Shaping::Advanced)).on_press(Message::Promotion(PieceState {owner, piece: ChessPiece::Knight}))
                            .style( |_,state| { custom_theme::ChessStyle::bright_button_wrapper(state) } ),
                        
                        button(text("♜").size(50.0).color(text_color).shaping(text::Shaping::Advanced)).on_press(Message::Promotion(PieceState {owner, piece: ChessPiece::Rook}))
                            .style( |_,state| { custom_theme::ChessStyle::dark_button_wrapper(state) } ),
                    ]
                ]
                .align_x(Alignment::Center)
            );

            modal::modal_no_skip(content, promotion_modal).into()

        } else if self.game_over {
            let winner = match self.turn {
                Player::Black => { String::from("White") },
                Player::White => { String::from("Black") },
            };

            let game_over_modal = container(
                text(format!("Win {}", winner)).size(50.0)
            ).style(container::rounded_box);

            modal::modal(content, game_over_modal, Message::Reset).into()

        } else {
            content.into()
        }
        
        
    }
}