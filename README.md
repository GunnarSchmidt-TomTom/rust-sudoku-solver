# rust-sudoku-solver
An educational project for me to learn Rust. Use it at your own risk ;)

# How to use it?
Just give it the path to a file containing a solvable, but unsolved 9x9 [sudoko](https://en.wikipedia.org/wiki/Sudoku) puzzle via command line.
```
$>cargo run src/example.txt
```

The files are expected to look like this:
```
|710|006|000|
|630|000|009|
|004|000|126|
----+---+----
|003|470|018|
|009|003|050|
|120|908|004|
----+---+----
|000|300|405|
|840|000|201|
|300|500|607|
```
note that the '-+|' characters will be ignored, they're just for pretty-printing. The unknown/empty digits are represented by 0.

# License
 Attribution 4.0 International (CC BY 4.0)  [![License: CC BY 4.0](https://licensebuttons.net/l/by/4.0/80x15.png)](https://creativecommons.org/licenses/by/4.0/)
