use iced::{
    Alignment, Application, Command, Element, Length, Settings, executor
};
use iced::theme::{self, Theme};
use iced::widget::{
    text, button, container, column, row, Column, Row,
};

pub fn main() -> iced::Result {
    Chess::run(Settings {
        ..Settings::default()
    })
}

const CHESS_LEHGT: usize = 8;


#[derive(Clone, Copy)]
struct TileState {
    piece_state: Option<PieceState>,
    high_light: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PieceState {
    owner: Owner,
    piece: ChessPiece,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Owner {
    White,
    Black,
}

#[derive(Clone, PartialEq, Copy, Debug)]
enum ChessPiece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}



#[derive(Debug, Clone)]
struct Point {
    h: usize,
    w: usize,
}

type Board = Vec<Vec<TileState>>;

struct Chess {
    board: Board,
    turn: Owner,
    choose: Option<(Point, PieceState)>,
}

#[derive(Debug, Clone)]
enum Message {
    MoveAble(Point, Option<PieceState>),
    Move(Point),
    Reset,
}

fn init_board() -> Board {
    let mut board: Board = vec![vec![TileState {piece_state: None, high_light: false}; CHESS_LEHGT]; CHESS_LEHGT];

    // ♜♞♝♛♚♝♞♜
    let chess_seq: [ChessPiece; 8] = [
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
        board[0][i] = TileState {piece_state: Some( PieceState {owner: Owner::Black , piece: chess_seq[i].clone() }), high_light: false};
        board[1][i] = TileState {piece_state: Some( PieceState {owner: Owner::Black , piece: ChessPiece::Pawn }), high_light: false};

        
        board[6][i] = TileState {piece_state: Some( PieceState {owner: Owner::White , piece: ChessPiece::Pawn }), high_light: false};
        board[7][i] = TileState {piece_state: Some( PieceState {owner: Owner::White , piece: chess_seq[i].clone() }), high_light: false};
    }

    board
}

fn view_tile(tile: &TileState, h: usize, w: usize) -> Element<Message>  {
    let b;
    let text_color;

    if let Some(x) = &tile.piece_state {
        match x.owner {
            Owner::White => { text_color = custom_theme::BTColor::White; },
            Owner::Black => { text_color = custom_theme::BTColor::Black; },
        };

        let t = match x.piece {
            ChessPiece::Bishop  => { text("♝") },
            ChessPiece::King    => { text("♚") },
            ChessPiece::Pawn    => { text("♟") },
            ChessPiece::Queen   => { text("♛") },
            ChessPiece::Rook    => { text("♜") },
            ChessPiece::Knight  => { text("♞") },
        };
        
        if tile.high_light == true {
            b = button(t.size(50.0).horizontal_alignment(iced::alignment::Horizontal::Center)).on_press(Message::Move(Point{ h, w }));
        } else {
            b = button(t.size(50.0).horizontal_alignment(iced::alignment::Horizontal::Center)).on_press(Message::MoveAble(Point{ h, w }, Some(*x)));
        }
        
    } else {

        if tile.high_light == true {
            b = button(" ").on_press(Message::Move(Point{ h, w }));
        } else {
            b = button(" ");
        }
        text_color = custom_theme::BTColor::White;
    }
    
    let background = match tile.high_light {
        true => { custom_theme::BColor::HighLight },
        false => {
            match (h, w) {
                (h, w) if (h % 2) ^ (w % 2) == 0 => {custom_theme::BColor::Bright},
                (_,_) => {custom_theme::BColor::Dark},
            }
        }
    };
    

    b.height(Length::Fixed(80.0)).width(Length::Fixed(80.0))
        .style(theme::Button::Custom( Box::new(custom_theme::Bbutton { background, text_color})))
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

fn move_able_pawn(board: &Board, p: &Point) -> Vec<Point> {
    let mut high_light: Vec<Point> = Vec::new();

    let h = p.h;
    let w = p.w;    

    let piece = board[h][w].piece_state.clone().unwrap();
    match piece.owner {
        Owner::Black => {
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
            }

            if w as i32 - 1 >= 0 {
                if let Some(x) = board[h + 1][w - 1].piece_state {
                    if x.owner != piece.owner {
                        high_light.push(Point { h: h + 1, w : w - 1});
                    }
                }
            }
        },
        Owner::White => {
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
            }

            if w as i32 - 1 >= 0 {
                if let Some(x) = board[h - 1][w - 1].piece_state {
                    if x.owner != piece.owner {
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

fn move_able(board: &Board, p: &Point, piece: ChessPiece) -> Vec<Point> {
    let move_able = match piece {
        ChessPiece::Rook    => { move_able_rook },
        ChessPiece::Bishop  => { move_able_bishop },
        ChessPiece::King    => { move_able_king },
        ChessPiece::Knight  => { move_able_knight },
        ChessPiece::Pawn    => { move_able_pawn },
        ChessPiece::Queen   => { move_able_queen },
    };

    move_able(&board, &p)
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


impl Application for Chess {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let board = init_board();

        (
            Self {
                board,
                turn: Owner::White,
                choose: None,
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

                    let light = move_able(&self.board, &point, piece_state.piece);
                    self.board = update_high_light(&self.board, light, false);
                }

                if let Some(x) = piece_state {
                    if x.owner == self.turn {
                        let light = move_able(&self.board, &p, x.piece);
                        self.board = update_high_light(&self.board, light, true);
                        self.choose = Some((p, x));
                    }
                }
                
                Command::none()
            },
            Message::Move(p) => {
                // let mut cloned_board: Board = self.board.iter()
                //     .map(|inner_vector| inner_vector.clone())
                //     .collect();
                
                if let Some((point, piece_state)) = &self.choose {
                
                    let light = move_able(&self.board, &point, piece_state.piece);
                    self.board = update_high_light(&self.board, light, false);

                    self.board[p.h][p.w] = self.board[point.h][point.w];
                    self.board[point.h][point.w] = TileState {piece_state: None, high_light: false};
                    self.choose = None;
                    
                    self.turn = match self.turn {
                        Owner::White => { Owner::Black },
                        Owner::Black => { Owner::White }
                    }
                }
                

                Command::none()
            },
            Message::Reset => {
                let board = init_board();

                self.board = board;
                self.turn = Owner::White;
                self.choose = None;

                Command::none()        
            },
        }
    }

    fn view(&self) -> Element<Message> {
        

        let board = (0..CHESS_LEHGT).into_iter().fold(Column::new() ,|c, i|
                c.push(Element::from(
                    (0..CHESS_LEHGT).into_iter().fold(Row::new().align_items(Alignment::Center) ,|c, j|
                        c.push(
                            view_tile(&self.board[i][j], i, j)
                        )
                    )
                ))
            );
        
        container(
            column!(
                button("reset").on_press(Message::Reset),
                board,
            )
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .center_x()
        .into()
        
    }
}




mod custom_theme {
    use iced::{Background, Border, Color, color, Shadow, Vector};

    use iced::widget::button;
    use iced::theme::Theme;

    #[derive(Default)]
    pub enum BColor {
        #[default]
        Bright,
        Dark,
        HighLight,
    }

    pub enum BTColor {
        White,
        Black,
    }

    pub struct Bbutton {
        pub background : BColor,
        pub text_color: BTColor,
    }


    impl button::StyleSheet for Bbutton {
        type Style = Theme;
    
        fn active(&self, _style: &Self::Style) -> button::Appearance {

            let background: Option<Background> = match self.background {
                BColor::Bright      => { Some(Background::Color(color!(0xf4, 0xdf, 0xc1))) },
                BColor::Dark        => { Some(Background::Color(color!(0xb6, 0x87, 0x6b))) },
                BColor::HighLight   => { Some(Background::Color(color!(0xFF, 0xFF, 0x0))) },
            };

            let text_color: Color = match self.text_color {
                BTColor::Black  => { color!(0x0, 0x0, 0x0) },
                BTColor::White  => { color!(0xFF, 0xFF, 0xFF) },
            };

            button::Appearance {
                shadow_offset: Vector::default(),
                background ,
                text_color,
                border: Border::with_radius(0),
                shadow: Shadow::default(),
            }

        }
    
        fn hovered(&self, _style: &Self::Style) -> button::Appearance {
            let background: Option<Background> = match self.background {
                BColor::Bright      => { Some(Background::Color(color!(0xf4, 0xdf, 0xc1, 0.6))) },
                BColor::Dark        => { Some(Background::Color(color!(0xb6, 0x87, 0x6b, 0.6))) },
                BColor::HighLight   => { Some(Background::Color(color!(0xcc, 0xc0, 0xb4, 0.6))) },
            };

            let text_color: Color = match self.text_color {
                BTColor::Black  => { color!(0x0, 0x0, 0x0) },
                BTColor::White  => { color!(0xFF, 0xFF, 0xFF) },
            };

            button::Appearance {
                shadow_offset: Vector::default(),
                background ,
                text_color,
                border: Border::with_radius(0),
                shadow: Shadow::default(),
            }
        }
    
        // fn pressed(&self, style: &Self::Style) -> button::Appearance {
        //     if let Button::Custom(custom) = style {
        //         return custom.pressed(self);
        //     }
    
        //     button::Appearance {
        //         shadow_offset: Vector::default(),
        //         ..self.active(style)
        //     }
        // }
    
        fn disabled(&self, style: &Self::Style) -> button::Appearance {
            self.active(style)
        }
    }
}