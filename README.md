# tic-tac-rust
## This wounderfull project
This project is a tic-tac-toe in Rust, you can play against your friends (if you got some), algos and it also allow to look 2 algos fighting !

## How can i use this god-created thing ?
To use this project launch
``` shell
cargo run -- Your flags
```
## Flags
There is 3 differents flags:
- [x] The `a` flag is used to launch a specific algorythm:
  - `-a 1` is a stupid algorythm -you should be able to beat him-
  - `-a 2` is a stupid algorythm but wont't let you win so easily
  - `-a 3` is a piramidal algorythm -good luck-
  - `-a 4` is a L algorythm
  - `-a 5` is an algorythm who play randomly
  - `-a 6` is a random algorythm between them all
- [x] The `f` flag allow you to fight 2 algorythm as same as before `-f 6`
- [x] The `p` flag allow you to choose your player -1 or 2-
## Error
You can catch error when playing:
- OutOfBox mean that you play out of the tic-tac-toe
- AlreadyFilled mean that you played on a case aready filled
