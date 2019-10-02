extern crate colored;
use std::boxed::*;
use self::colored::*;
use std::fmt;

#[derive(Debug)]
pub enum GrillErr {
    OutOfBox,
    AlreadyFilled
}

#[derive(Debug)]
pub struct Grill {
    pub cells: Box<[char]>,
    player: u8,
}

impl fmt::Display for Grill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        let mut str: String = String::new();
        for cell in self.cells.iter() {
            let mut print: String = ".".to_string();
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

    pub fn play(&mut self, pos: usize) -> Result<usize,GrillErr> {
        if pos - 1 >= 9 || pos - 1 < 0 {
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

    fn check_line(self, pos: usize) -> u8 {
        if pos % 3 != 0 {
            return 0
        }
                let all:(&char, &char, &char) = (
                    self.cells.get(pos).unwrap(),
                    self.cells.get(pos + 1).unwrap(),
                    self.cells.get(pos + 2).unwrap()
                    );
                match all {
                    ('X', 'X', 'X') => 1,
                    ('O', 'O', 'O') => 2,
                    _ => 0,
                }
    }

    pub fn win(&mut self) -> u8 {
        let mut i: usize = 0;
        for cell in self.cells.iter() {
            self.check_line(i);
            //    self.check_column(i);
            //  self.check_diagonal(i);
            i = i + 1;
        }
        0
    }
}

use std::io::{stdin};

fn main() {
    println!("Hello, world!");
    let mut grill = Grill::new(1);
    loop {
        let mut s = String::new();
        println!("{}", grill);

        println!("Give me your position pls: ");
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        let pos = match s.parse::<usize>() {
            Ok(i) => i,
            Err(e) => {
                0
            }
        };
        if pos == 0 {
            println!("Error: Bad input (OutOfBox)");
            continue ;
        }
        match grill.play(pos) {
            Err(er) => println!("Error: Bad input ({:?})", er),
            Ok(_l) => println!("Works"),
        }
        if grill.win() != 0 {
            break ;
        }

    }
    println!("{}", grill);

}
