use iced::{
    Alignment, Application, Command, Element, Length, Settings, 
    Subscription, executor,
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

/* 
    ♔ ♚
    ♕/♛
    ♖/♜
    ♗/♝
    ♘/♞
    ♙/♟
*/

enum TileColor {
    White,
    Black,
    HighLight,
}

#[derive(Clone)]
struct TileState {
    owner: Owner,
    piece: ChessPiece,
}

#[derive(Clone)]
enum Owner {
    White,
    Black,
}

#[derive(Clone)]
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

type Board = Vec<Vec<Option<TileState>>>;

struct Chess {
    board: Board,
    turn: Owner,
    choose: Option<Point>,
}

#[derive(Debug, Clone)]
enum Message {
    MoveAble(Point),
    Move(Point),
    Reset,
}

fn init_board() -> Board {
    let mut board: Board = vec![vec![None ; CHESS_LEHGT]; CHESS_LEHGT];

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
        board[0][i] = Some( TileState {owner: Owner::Black , piece: chess_seq[i].clone() });
        board[1][i] = Some( TileState {owner: Owner::Black , piece: ChessPiece::Pawn });
        
        board[6][i] = Some( TileState {owner: Owner::White , piece: ChessPiece::Pawn });
        board[7][i] = Some( TileState {owner: Owner::White , piece: chess_seq[i].clone() });
    }

    board
}

fn view_tile(tile: &Option<TileState>, h: usize, w: usize) -> Element<Message>  {
    let b;
    let text_color;

    if let Some(x) = tile {
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
        
        b = button(t.size(50.0).horizontal_alignment(iced::alignment::Horizontal::Center)).on_press(Message::MoveAble(Point{ h, w }));
    } else {
        b = button(" ");
        text_color = custom_theme::BTColor::White;
    }
    
    let background = match (h, w) {
        (h, w) if (h % 2) ^ (w % 2) == 0 => {custom_theme::BColor::Bright},
        (_,_) => {custom_theme::BColor::Dark},
    };

    b.height(Length::Fixed(80.0)).width(Length::Fixed(80.0))
        .style(theme::Button::Custom( Box::new(custom_theme::Bbutton { background, text_color})))
        .into()   
}


fn move_able(board: &Board, p: Point) -> Board {
    let mut cloned_board: Board = board.iter()
        .map(|inner_vector| inner_vector.clone())
        .collect();

    let t = cloned_board[p.h][p.w].clone().unwrap();
    match t.piece {
        ChessPiece::Pawn => {},
        ChessPiece::Bishop => {},
        ChessPiece::King => {},
        ChessPiece::Knight => {},
        ChessPiece::Queen => {},
        ChessPiece::Rook => {
            
        },
    };


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
            Message::MoveAble(p) => {

                if let Some(t) = &self.board[p.h][p.w] {

                }



                Command::none()
            },
            Message::Move(p) => {
                Command::none()
            },
            Message::Reset => {
                let board = init_board();

                Self {
                    board,
                    turn: Owner::White,
                    choose: None,
                };
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<Message> {
        

        (0..CHESS_LEHGT).into_iter().fold(Column::new() ,|c, i|
                c.push(Element::from(
                    (0..CHESS_LEHGT).into_iter().fold(Row::new().align_items(Alignment::Center) ,|c, j|
                        c.push(
                            view_tile(&self.board[i][j], i, j)
                        )
                    )
                ))
            ).into()
        
    }
}




mod custom_theme {
    // use iced::{color, Background, Border, Shadow};
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
                BColor::HighLight   => { Some(Background::Color(color!(0xcc, 0xc0, 0xb4))) },
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