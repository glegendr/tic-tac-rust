extern crate colored;
use std::boxed::*;
use self::colored::*;
use std::fmt;
use std::io::{stdin};
extern crate structopt;
use structopt::StructOpt;
extern crate rand;

use rand::Rng;


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

    fn get_line(&mut self, pos: usize) -> [&char; 3] {
        if pos % 3  == 0 {
            return [
                self.cells.get(pos).unwrap(),
                self.cells.get(pos + 1).unwrap(),
                self.cells.get(pos + 2).unwrap()
            ]
        } else if pos % 3 == 1 {
            return [
                self.cells.get(pos - 1).unwrap(),
                self.cells.get(pos).unwrap(),
                self.cells.get(pos + 1).unwrap()
            ]
        } else {
            return [
                self.cells.get(pos - 2).unwrap(),
                self.cells.get(pos - 1).unwrap(),
                self.cells.get(pos).unwrap()
            ]
        }
    }

    fn check_line(&mut self, pos: usize) -> u8 {
        if pos % 3 != 0 {
            return 0
        }
        match self.get_line(pos) {
            ['X', 'X', 'X'] => 1,
            ['O', 'O', 'O'] => 2,
            _ => 0,
        }
    }

    fn get_column(&mut self, pos: usize) -> [&char; 3] {
        if pos < 3 {
            return [
                self.cells.get(pos).unwrap(),
                self.cells.get(pos + 3).unwrap(),
                self.cells.get(pos + 6).unwrap()
            ]
        } else if pos < 6 {
            return [
                self.cells.get(pos - 3).unwrap(),
                self.cells.get(pos).unwrap(),
                self.cells.get(pos + 3).unwrap()
            ]
        } else {
            return [
                self.cells.get(pos - 6).unwrap(),
                self.cells.get(pos - 3).unwrap(),
                self.cells.get(pos).unwrap()
            ]
        }
    }

    fn check_column(&mut self, pos: usize) -> u8 {
        if pos >= 3 {
            return 0
        }
        match self.get_column(pos) {
            ['X', 'X', 'X'] => 1,
            ['O', 'O', 'O'] => 2,
            _ => 0,
        }
    }

    fn get_diagonal(&mut self, pos: usize) -> [&char; 3] {
        if pos % 2 == 1 {
            return [&'0', &'0', &'0']
        }
        if pos == 0 || pos == 4 || pos == 8 {
            return [
                self.cells.get(0).unwrap(),
                self.cells.get(4).unwrap(),
                self.cells.get(8).unwrap()
            ]
        } else {
            return [
                self.cells.get(2).unwrap(),
                self.cells.get(4).unwrap(),
                self.cells.get(6).unwrap()
            ]
        }
    }

    fn check_diagonal(&mut self, pos: usize) -> u8 {
        if pos != 0 && pos != 2 {
            return 0
        }
        match self.get_diagonal(pos) {
            ['X', 'X', 'X'] => 1,
            ['O', 'O', 'O'] => 2,
            _ => 0,
        }
    }

    fn get_corner(&mut self) -> [&char; 4] {
        return [
            self.cells.get(0).unwrap(),
            self.cells.get(2).unwrap(),
            self.cells.get(6).unwrap(),
            self.cells.get(8).unwrap(),
        ]
    }

    fn get_grill(&mut self) -> [&char; 9] {
        [
            self.cells.get(0).unwrap(),
            self.cells.get(1).unwrap(),
            self.cells.get(2).unwrap(),
            self.cells.get(3).unwrap(),
            self.cells.get(4).unwrap(),
            self.cells.get(5).unwrap(),
            self.cells.get(6).unwrap(),
            self.cells.get(7).unwrap(),
            self.cells.get(8).unwrap(),
        ]
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
            return i
        }
    }
    0
}

fn medium_algo(my_grill: &mut Grill, player: usize) -> usize {
    let mut win = can_i_win(my_grill, player);
    if win < 10 {
        return win
    }
    if player ==  1 {
        win = can_i_win(my_grill, 2);
    } else {
        win = can_i_win(my_grill, 1);
    }
    if win < 10 {
        return win
    }
    for i in 0..9 {
        if *my_grill.cells.get(i).unwrap() == ' ' {
            return i
        }
    }
    0
}

fn free_space(array: [&char; 3], player: usize) -> usize {
    let my_char: &&char;
    let mut space = 10;
    if player == 1 {
        my_char = &&'X';
    } else {
        my_char = &&'O';
    }
    let mut i = 0;
    for ch in array.iter() {
        if ch != my_char && ch != &&' ' {
            return 11
        } else if space != 10 && ch == &&' ' {
            return 10
        } else if ch == &&' ' {
            space = i;
        }
        i = i + 1;
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

fn aim_corner(my_grill: &mut Grill, player: usize) -> usize {
    match my_grill.get_corner() {
        [' ', ' ', ' ', ' '] => return 0,
        _ => ()
    };
    let mut line = 10;
    let mut col = 10;
    let mut i = 0;
    for ch in my_grill.clone().get_corner().iter() {
        if ch != &&' ' {
            i = i + 1;
            continue ;
        }
        if player == 2 {
            match my_grill.get_line(if i < 2 {i * 2} else {2 + i * 2}) {
                [_, 'X', _] => { i = i + 1; continue } ,
                _ => line = i
            }
            match my_grill.get_column(if i < 2 {i * 2} else {2 + i * 2}) {
                [_, 'X', _] => { i = i + 1;if line == i {line = 10}; continue },
                _ => col = i
            }
        } else {
            match my_grill.get_line(if i < 2 {i * 2} else {2 + i * 2}) {
                [_, 'O', _] => { i = i + 1;continue } ,
                _ => line = i
            }
            match my_grill.get_column(if i < 2 {i * 2} else {2 + i * 2}) {
                [_, 'O', _] => { i = i + 1;if line == i {line = 10}; continue },
                _ => col = i
            }
        }
        if line == col {
            return if i < 2 {i * 2} else {2 + i * 2}
        }
        i = i + 1;
    }
    if line != 10 {
        return if line < 2 {line * 2} else {2 + line * 2}
    } else if col != 10 {
        return if col < 2 {col * 2} else {2 + col * 2}
    }
    10
}

fn aim_oposit_corner(my_grill: &mut Grill, player: usize) ->usize {
    match my_grill.get_grill() {
        ['X', ' ', ' ', ' ', 'O', ' ', ' ', ' ', ' '] => return 8,
        [' ', ' ', 'X', ' ', 'O', ' ', ' ', ' ', ' '] => return 5,
        _ => ()
    }
    return aim_corner(my_grill, player)
}

fn place(my_grill: &mut Grill, player: usize) -> usize {
    match my_grill.get_grill() {
        ['X', ' ', ' ', ' ', 'O', ' ', ' ', ' ', 'X'] => return 1,
        [' ', ' ', 'X', ' ', 'O', ' ', 'X', ' ', ' '] => return 1,
        _ => ()
    }
    let ret = aim_corner(my_grill, player);
    if ret != 10 {
        return ret
    }
    return stupid_algo(my_grill)
}

fn basic_algo(my_grill: &mut Grill, player: usize) -> usize {
    let mut win = can_i_win(my_grill, player);
    if win < 10 {
        return win
    }
    if player ==  1 {
        win = can_i_win(my_grill, 2);
    } else {
        win = can_i_win(my_grill, 1);
    }
    if win < 10 {
        return win
    } else if *my_grill.cells.get(4).unwrap() == ' ' {
        return 4
    }
    place(my_grill, player)
}

fn l_algo(my_grill: &mut Grill, player: usize) -> usize {
    let mut win = can_i_win(my_grill, player);
    if win < 10 {
        return win
    }
    if player ==  1 {
        win = can_i_win(my_grill, 2);
    } else {
        win = can_i_win(my_grill, 1);
    }
    if win < 10 {
        return win
    } else if player == 1 {
        let win = aim_oposit_corner(my_grill, player);
        if win < 10 {
            return win
        }
    }
    return basic_algo(my_grill, player)
}

fn random_algo(my_grill: &mut Grill) -> usize {
    loop {
        let mut rng = rand::thread_rng();
        let pos = rng.gen_range(0, 9);
        if my_grill.cells.get(pos).unwrap() == &' ' {
            return pos;
        }
    }
}

fn choose_algo(algo_id: usize, grill: &mut Grill, player: usize) -> usize {
    if algo_id == 1 {
        return stupid_algo(grill) + 1
    } else if algo_id == 2 {
        return medium_algo(grill, player) + 1
    } else if algo_id == 3 {
        return basic_algo(grill, player) + 1
    } else if algo_id == 4 {
        return l_algo(grill, player) + 1
    }
    return random_algo(grill) + 1
}

#[derive(Debug, StructOpt)]
#[structopt(name = "tic-tac-parsing")]
struct Opt {
    /// allow to play with an algo
    /// algo:
    /// - 1 is a stupid algo
    /// - 2 is a medium algo
    /// - 3 is a piramidal algo
    /// - 4 is a L algo
    /// - 5 is a random algo
    /// - 6 is a random between them all -
#[structopt(short = "a", long = "algo_id", default_value = "0")]
    algo_id: usize,
    /// allow to fight 2 algo, there are the sames as `a` flag
    #[structopt(short = "f", long = "algo_fight", default_value = "0")]
    algo_fight: usize,
    /// This mean if you want to play player 1 or 2
    #[structopt(short = "p", long = "player", default_value = "1")]
    player: u8,
}

fn play_and_print(grill: &mut Grill, pos: usize) -> bool {
    match grill.play(pos) {
        Err(er) => println!("Error: Bad input ({:?})\n1|2|3\n4|5|6\n7|8|9\n------", er),
        Ok(_) => (),
    }
    let win = grill.win();
    if win != 0 {
        println!("-----[ Player {} Won ]-----", if win == 1 { ("1".yellow().bold()) } else { ("2".red().bold()) });
        return true
    } else if grill.is_filled() {
        println!("-----[ {} ]-----", "Draw".purple().bold());
        return true
    }
    println!("{}", grill);
    false
}

const ALGO_NBR: usize = 5;

fn main() {
    let mut opt = Opt::from_args();
    let mut grill = Grill::new(1);
    let mut pos;
    println!("-----[ Play With Numeric Keypad ]-----\n7|8|9\n4|5|6\n1|2|3");
    loop {
        let mut s = String::new();

        if (opt.algo_id == 0 || grill.player == opt.player) && opt.algo_fight == 0 {
            println!("Player {} Turn ", if grill.player == 1 { ("1".yellow().bold()) } else { ("2".red().bold()) });
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
            if pos <= 3 {
                pos += 6;
            } else if pos >= 7 {
                pos -= 6;
            }
        } else {
            if opt.algo_id < 1 || opt.algo_id > ALGO_NBR {
                let mut rng = rand::thread_rng();
                opt.algo_id =  rng.gen_range(1, 5);
            }
            pos = choose_algo(opt.algo_id, &mut grill, if opt.player == 1 { 2 } else { 1 });
            println!("Algorythm {} Turn", if grill.player == 1 { ("1".yellow().bold()) } else { ("2".red().bold())});
        }
        if play_and_print(&mut grill, pos) == true {
            break ;
        } else if opt.algo_fight != 0 {
            if opt.algo_fight < 1 || opt.algo_fight > ALGO_NBR {
                let mut rng = rand::thread_rng();
                opt.algo_fight =  rng.gen_range(1, 5);
            }
            pos = choose_algo(opt.algo_fight, &mut grill, if opt.player == 1 { 2 } else { 1 });
            println!("Algorythm {} Turn", "2".red().bold());
            if play_and_print(&mut grill, pos) == true {
                break ;
            }
        }
    }
    println!("{}", grill);
}
