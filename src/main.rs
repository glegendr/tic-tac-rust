extern crate colored;
use std::boxed::*;
use self::colored::*;
use std::fmt;
use std::io::{stdin};
extern crate structopt;
use structopt::StructOpt;

#[derive(Debug)]
pub enum GrillErr {
    OutOfBox,
    AlreadyFilled
}

#[derive(Debug, Clone)]
pub struct Grill {
    pub cells: Box<[char]>,
    player: u8,
}

impl fmt::Display for Grill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        let mut str: String = String::new();
        for cell in self.cells.iter() {
            let print: String;
            print = format!("{}", cell);
            if print == "X" {
                str.push_str(format!("{}", print.yellow().bold()).as_str());
            } else if print == "O" {
                str.push_str(format!("{}", print.red().bold()).as_str());
            } else {
                str.push(' ');
            }
            i = i + 1;
            if i % 3 == 0 && i != 9{
                str.push_str("\n");
            } else if i != 9 {
                str.push('|');
            }
        }
        write!(f, "{}", str)
    }
}


impl Grill {
    pub fn new(player_id: u8) -> Self  {
        let mut vec: Vec<char> = Vec::new();
        vec.extend([' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '].iter().clone());
        return Self {cells: vec.into_boxed_slice(), player: player_id}
    }

    pub fn is_filled(&mut self) -> bool {
        for cell in self.cells.iter() {
            if cell == &' ' {
                return false
            }
        }
        true
    }

    pub fn play(&mut self, pos: usize) -> Result<usize,GrillErr> {
        if pos > 9 || pos < 1 {
            return Err(GrillErr::OutOfBox)
        } else if let Some(x) = self.cells.get_mut((pos - 1) as usize) {
            if *x != ' ' {
                return Err(GrillErr::AlreadyFilled)
            }
            if self.player == 1 {
                *x = 'X';
                self.player = 2;
            } else {
                *x = 'O';
                self.player = 1;
            }
        }
        Ok(pos)
    }

    fn get_line(&mut self, pos: usize) -> (&char, &char, &char) {
        if pos % 3  == 0 {
            return (
                self.cells.get(pos).unwrap(),
                self.cells.get(pos + 1).unwrap(),
                self.cells.get(pos + 2).unwrap()
            );
        } else if pos % 3 == 1 {
            return (
                self.cells.get(pos - 1).unwrap(),
                self.cells.get(pos).unwrap(),
                self.cells.get(pos + 1).unwrap()
            );
        } else {
            return (
                self.cells.get(pos - 1).unwrap(),
                self.cells.get(pos - 2).unwrap(),
                self.cells.get(pos).unwrap()
            );
        }
    }

    fn check_line(&mut self, pos: usize) -> u8 {
        if pos % 3 != 0 {
            return 0
        }
        match self.get_line(pos) {
            ('X', 'X', 'X') => 1,
            ('O', 'O', 'O') => 2,
            _ => 0,
        }
    }

    fn get_column(&mut self, pos: usize) -> (&char, &char, &char) {
        if pos < 3 {
            return (
                self.cells.get(pos).unwrap(),
                self.cells.get(pos + 3).unwrap(),
                self.cells.get(pos + 6).unwrap()
            );
        } else if pos < 6 {
            return (
                self.cells.get(pos - 3).unwrap(),
                self.cells.get(pos).unwrap(),
                self.cells.get(pos + 3).unwrap()
            );
        } else {
            return (
                self.cells.get(pos - 6).unwrap(),
                self.cells.get(pos - 3).unwrap(),
                self.cells.get(pos).unwrap()
            );
        }
    }

    fn check_column(&mut self, pos: usize) -> u8 {
        if pos >= 3 {
            return 0
        }
        match self.get_column(pos) {
            ('X', 'X', 'X') => 1,
            ('O', 'O', 'O') => 2,
            _ => 0,
        }
    }

    fn get_diagonal(&mut self, pos: usize) -> (&char, &char, &char) {
        if pos % 2 == 1 {
            return (&'0', &'0', &'0')
        }
        if pos == 0 || pos == 4 || pos == 8 {
            return (
                self.cells.get(0).unwrap(),
                self.cells.get(4).unwrap(),
                self.cells.get(8).unwrap()
            )
        } else {
            return (
                self.cells.get(2).unwrap(),
                self.cells.get(4).unwrap(),
                self.cells.get(6).unwrap()
            )
        }
    }

    fn check_diagonal(&mut self, pos: usize) -> u8 {
        if pos != 0 && pos != 2 {
            return 0
        }
        match self.get_diagonal(pos) {
            ('X', 'X', 'X') => 1,
            ('O', 'O', 'O') => 2,
            _ => 0,
        }
    }

    pub fn win(&mut self) -> u8 {
        let mut ret;
        for i in 0..9 {
            ret = self.check_line(i);
            if ret != 0 {
                return ret
            }
            ret = self.check_column(i);
            if ret != 0 {
                return ret
            }
            ret = self.check_diagonal(i);
            if ret != 0 {
                return ret
            }
        }
        0
    }
}

fn stupid_algo(grill: &mut Grill) -> usize {
    for i in 0..9 {
        if *grill.cells.get(i).unwrap() == ' ' {
            return i + 1
        }
    }
    0
}

fn free_space(array: (&char, &char, &char), player: usize) -> usize {
    let mut my_char: &char;
    let mut space = 10;
    if player == 1 {
        my_char = &'X';
    } else {
        my_char = &'O';
    }
    for i in 0..3 {
        let ch = array.0;
        if ch != my_char && ch != &' ' {
            return 11
        } else if space != 10 && ch == &' ' {
            return 10
        } else if ch == &' ' {
            space = i;
        }
    }
    space
}

fn can_i_win(grill: &mut Grill, player: usize) -> usize {
    let mut ret;
    for i in 0usize..9 {
        ret = free_space(grill.get_column(i), player);
        if ret < 10 {
            return i + 3 * ret
        }
        ret = free_space(grill.get_line(i), player);
        if ret < 10 {
            return i + ret
        }
        ret = free_space(grill.get_diagonal(i), player);
        if ret < 10 {
            if i == 2 || i == 6 {
                return i + 2 * ret
            } else {
                return i + 4 * ret
            }
        }
    }
    10
}

//fn place(my_grill: &mut Grill, player: usize) -> usize {
//}

fn basic_algo(my_grill: &mut Grill, player: usize) -> usize {
    let win = can_i_win(my_grill, player);
    if win < 10 {
        return win
    }
    if player ==  1 {
        let win = can_i_win(my_grill, 2);
    } else {
        let win = can_i_win(my_grill, 1);
    }
    if win < 10 {
        return win
    }
    if *my_grill.cells.get(4).unwrap() == ' ' {
        return 4 + 1
    }
    //place(my_grill, player)
    0
}

fn choose_algo(algo_id: usize, grill: &mut Grill, player: usize) -> usize {
    if algo_id == 1 {
        return stupid_algo(grill)
    } else {
        return basic_algo(grill, player)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "tic-tac-parsing")]
struct Opt {
    /// allow to play with an algo
#[structopt(short = "a", long = "algo_id", default_value = "0")]
    algo_id: usize,
#[structopt(short = "p", long = "player", default_value = "2")]
    player: usize,
}

fn main() {
    let mut grill = Grill::new(1);
    let opt = Opt::from_args();
    let mut pos;
    loop {
        let mut s = String::new();
        println!("{}", grill);

        if opt.algo_id == 0 || grill.player == 1 {
            println!("Give me your position pls: ");
            stdin().read_line(&mut s).expect("Did not enter a correct string");
            if let Some('\n')=s.chars().next_back() {
                s.pop();
            }
            if let Some('\r')=s.chars().next_back() {
                s.pop();
            }
            pos = match s.parse::<usize>() {
                Ok(i) => i,
                Err(_) => 0
            };
        } else {
            pos = choose_algo(opt.algo_id, &mut grill, if opt.player == 1 { 2 } else { 1 });
        }
        if pos == 0 {
            println!("Error: Bad input (OutOfBox)");
            continue ;
        }
        match grill.play(pos) {
            Err(er) => println!("Error: Bad input ({:?})", er),
            Ok(_) => (),
        }
        let win = grill.win();
        if win != 0 {
            println!("-----[ Player {} Won ]-----", if win == 1 { ("1".yellow().bold()) } else { ("2".red().bold()) });
            break ;
        } else if grill.is_filled() {
            println!("-----[ {} ]-----", "Draw".purple().bold());
            break ;
        }

    }
    println!("{}", grill);
}
