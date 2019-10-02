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
                str.push_str(format!("{} ", print.yellow().strikethrough().bold()).as_str());
                str.push_str(format!("{} ", print.red().bold()).as_str());
            i = i + 1;
            if i % 3 == 0{
                str.push('\n');
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

    pub fn play(&mut self, pos: u8) -> Result<u8,GrillErr> {
        if pos > 9 {
            return Err(GrillErr::OutOfBox)
        } else if let Some(x) = self.cells.get_mut(pos as usize) {
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
}

fn main() {
    println!("Hello, world!");
    let mut grill =  Grill::new(1);
    match grill.play(6) {
        Err(er) => println!("Error: Bad input ({:?})", er),
        Ok(_l) => println!("Works"),
    }
}
