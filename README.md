# monorail

This is a small program written to analyze the game of Monorail played in [The Genius](https://en.wikipedia.org/wiki/The_Genius_%28TV_series%29).
Monorail is alternately known as [Venice Connection](https://boardgamegeek.com/boardgame/1910/venice-connection) by Alex Randolph.
A Kickstarter version of this game is known as [Racing Stripe](https://www.kickstarter.com/projects/918748661/deduction-and-racing-stripe) by Scott Green.

Unfortunately due to time constraints and the potentially wide search space, the generic version of the game is not implemented.
Instead, this implementation specifically focuses on the Monorail game played in season 4.
This was intended as a tool to determine whether the player that lost that game could actually have won.
Further details are not provided here so as to avoid spoilers for those who have not watched the relevant episode.
The initial game state is the board at the end of the first move of that game.

# Usage

This project comes in two parts, a generic `monorail` library and a small `main` program that uses the library.

`cargo run` will run the `main` wrapper.
The following flags are supported:

* `-b`: Print best move of the player to move and the game result under optimal play by both players after that move.
     It is probably best to run this in release mode, since the search can be rather time-consuming.
* `-a`: For all possible moves of the player to move, print what the opponent's best response is and the game result under optimal play by both players.
     It is probably best to run this in release mode, since the searches can be ratehr time-consuming.
* `-l`: List all legal moves of the player to move.
* `-c`: Colorize output.

If one (or more) of `-a`, `-b`, or `-l` are provided, the program performs the requested function(s) and then exits.

Otherwise, an interactive prompt is started.
At each step, the possible moves of the active player are listed,
Possible commands are:

* "analyze" or "a": The same as the `-a` flag.
* "best" or "b": The same as the `-b` flag.
* "undo" or "u": Undo the most recent move. (Yes, you may undo multiple times if desired)
* (A number): Perform the move labeled with that number.

## Board arrangement

Note that the board is entirely determined by the first move, except for the lower-left corner which has three possible arrangements.
The three possible arrangements are termed Left, Middle, and Right.
Some moves will force the board state into one of these three arrangements.
If the board is in such a state, moves that are illegal under that arrangement will not be allowed.
Some other moves will force the board state into two of the three arragements, precluding the third.
If the board is in such a state, moves that are only legal under the precluded arrangement will not be allowed.

## Example

An example of what it looks like (with the player name censored so as to avoid spoilers):

```
=================== Turn 4 ===================
     0 1 2 3 4
    ┌─┬─┬─┬─┬─┐
 0  │╔│═│═│═│╗│
    ├─┼─┼─┼─┼─┤
 1  │║│ │╔│═│╝│
    ├─┼─┼─┼─┼─┤
 2  │║│ │╚│═│╗│
    ├─┼─┼─┼─┼─┤
 3  │ │ │═│ │ │
    └─┴─┴─┴─┴─┘

0 Single at (row 3, col 0)
1 OneRight at (row 3, col 0)
2 Single at (row 3, col 1)
3 OneLeft at (row 3, col 1)
4 Single at (row 3, col 3)
5 OneRight at (row 3, col 3)
6 Single at (row 3, col 4)
7 OneLeft at (row 3, col 4)
It's <censored>'s turn. What move?
```
