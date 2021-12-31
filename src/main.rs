const LEN_Y: usize = 100;
const LEN_X: usize = 100;

#[derive(Clone)]
enum Player {
    X,
    O
}

#[derive(Clone, PartialEq)]
enum Field {
    O,
    X,
    N
}

enum Outcome {
    Xwin,
    Owin,
    None
}

enum Move {
    Valid,
    OutOfRange,
    AlreadyAssigned
}

struct Board {
    x: usize,
    y: usize,
    board: Vec<Vec<Field>>
}

impl Board {
    fn new(x: usize, y: usize) -> Board {
        if (x < 3) || (y < 3) {
            panic!("Size must be atleast 3x3")
        }

        let mut board: Vec<Vec<Field>> = vec![];
        for _i in 0..y {
                board.push(vec![Field::N; x])
        }
        Board{x, y, board}
    }

    fn display(self: &Self) {
        for line in &self.board {
            for item in line {
                match item {
                    Field::O => print!("0"),
                    Field::X => print!("X"),
                    Field::N => print!("#"),
                }
                print!(" ")
            }
            println!()
        }
        println!("{}", (0..(self.x*2)-1).map(|_| "=").collect::<String>())
    }

    fn is_field_win(self: &Self, x: usize, y: usize) -> bool {
        if self.board[x][y] == Field::N {
            return false
        }
        let base = self.board[x][y].clone();

        if (x != 0) && (x != self.x-1 ) {
            if self.board[x-1][y] == base && self.board[x+1][y] == base {
                return true
            }
        }

        if (y != 0) && (y != self.y-1 ) {
            if self.board[x][y-1] == base && self.board[x][y+1] == base {
                return true
            }
        }

        return false
    }

    fn calculate(self: &Self) -> Outcome {
        for row in 0..self.board.len() {
            for column in 0..self.board[0].len() {
                if self.is_field_win(row, column) {
                    match self.board[row][column] {
                        Field::X => return Outcome::Xwin,
                        Field::O => return Outcome::Owin,
                        Field::N => () // can't ever be reached
                    }
                }
            }
        }
        return Outcome::None
    }

    fn play(self: &mut Self, x: usize, y: usize, player: Player) -> Move {
        if (x > self.x) || (y > self.y) {
            return Move::OutOfRange
        }
        else if self.board[x][y] != Field::N {
            return Move::AlreadyAssigned
        }

        self.board[x][y] = match player {
            Player::X => Field::X,
            Player::O => Field::O
        };
        Move::Valid
    }

}

fn get_move() -> Result<(usize, usize), &'static str> {
    let x: usize;
    let y: usize;

    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    x = match trimmed.parse::<usize>() {
        Ok(v) => v,
        Err(..) => return Err("Invalid input")
    };


    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    y = match trimmed.parse::<usize>() {
        Ok(v) => v,
        Err(..) => return Err("Invalid input")
    };

    Ok((x, y))
}

fn main() {
    let mut player = Player::X;
    let mut playing = true;
    let mut b = Board::new(LEN_X, LEN_Y);

    while playing {
        println!("enter move for {}", match player {
            Player::O => String::from("O"),
            Player::X => String::from("X")
        });
        let (x, y) = match get_move() {
            Ok(v) => v,
            Err(e) => { eprintln!("{}", e); continue }
        };
        match b.play(x, y, player.clone()) {
            Move::Valid => (),
            Move::OutOfRange => { println!("out of range"); continue },
            Move::AlreadyAssigned => { println!("already ocupied"); continue; }
        }

        b.display();

        match b.calculate() {
            Outcome::Xwin => {println!("X wins"); playing=false},
            Outcome::Owin => {println!("O wins"); playing=false},
            Outcome::None => ()
        }
        player = match player {
            Player::O => Player::X,
            Player::X => Player::O
        }
    }
}
