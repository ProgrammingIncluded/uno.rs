# Uno.rs

A Uno simulator written in Rust. Used to determining Uno optimal strategies.

## How to Run

If running from the repo, you can use the default cargo manager:

```
cargo run --release -- --help
Options:
  -p, --players <PLAYERS>      Number of players. [default: 2]
  -n, --num-decks <NUM_DECKS>  Number of uno decks to play with, there are 108 cards per game. [default: 1]
  -x, --hand-size <HAND_SIZE>  Hand size during the game. [default: 7]
  -s, --seed <SEED>            Seed to play the game. [default: 0]
  -h, --help                   Print help
  -V, --version                Print version
```

Default behavior starts an interactive game and can be tweaked:

```
cargo run --release -- --hand-size 2
--- Playing as Hand #0 ---
Field Top: V1Y
Cards in Deck: 103
Cards in Field: 1

Hand #0: *-*, V2R
Hand #1: V5Y, V2R

0: H0R, 1: H0Y, 2: H0B, 3: H0G

Input a valid move from 0 - 3
0

--- Playing as Hand #1 ---
Field Top: *-R
Cards in Deck: 103
Cards in Field: 2

Hand #0: V2R
Hand #1: V5Y, V2R

0: H1R

Input a valid move from 0 - 0
0

--- Playing as Hand #0 ---
Field Top: V2R
Cards in Deck: 103
Cards in Field: 3

Hand #0: V2R
Hand #1: V5Y

0: H0R

Input a valid move from 0 - 0
0

Player 0 has won the game!
```

## Interpretting the IR

To keep the game succinct in terminal, an IR is used for display cards.

A general notation is as follows:

* <Card Type><Card Value><Card Color>

Each card can be encoded using this approach for example:

* Value Card: V<Value><Color>
* Referring to Hand: H<Hand_idx><Color>
* Draw Card: D<Number_Cards><Color>

Generally dashes denote non-value or irrelevant value.
Here are some examples of the IR:

* Wild Card: *-*
* Wild Card Played with Color: *-R
* Draw Four without Color Selected: D4* 
* Draw Four with Color Selected: D4R

## Custom Uno Games

The script allows for tweaking of common configuration parameters.
Upon specifying `--players` for example, number of decks being used will scale.
Here is an example of a 32 player game:

```
$> cargo run -- --players 32
Number of players exceed number of cards available if each player has 7 cards per hand.
Auto scaling num of decks to 3 for 32 players.
--- Playing as Hand #0 ---
Field Top: V4G
Cards in Deck: 207
Cards in Field: 1

Hand #0: *-*, V5B, *-*, D4*, V7R, V7B, V9Y
Hand #1: V1B, V3R, V8R, V2Y, V9B, R-Y, D2Y
Hand #2: V6Y, D4*, C-B, V1B, V1B, V8B, C-B
Hand #3: V9R, V8B, V6B, C-Y, V0B, V4Y, C-Y
Hand #4: R-G, V3Y, V2Y, C-Y, D4*, V7Y, V6Y
Hand #5: R-Y, C-G, V8Y, C-G, V1B, V4B, V6R
Hand #6: V2B, V7B, V1R, V6R, V2R, C-G, V6G
Hand #7: D4*, V7G, V5Y, V1B, V2R, V5Y, V6R
Hand #8: R-R, V1G, C-R, R-R, V1Y, V4G, C-G
Hand #9: V7B, V9R, V5G, V5B, V8B, V9R, V3B
Hand #10: V8G, D2G, V7R, V7B, V2R, V2G, V9G
Hand #11: *-*, V7R, V5G, C-B, V1R, V7B, V8B
Hand #12: V2B, V8R, V9R, V8B, V1G, V6R, R-B
Hand #13: V8R, V2R, V8B, V7Y, V2G, C-R, R-Y
Hand #14: V7Y, V2Y, V8G, D2B, D2G, C-Y, D2Y
Hand #15: V1Y, V7Y, V9B, V7R, V9R, V0B, V2Y
Hand #16: V4B, V1Y, D2B, V3B, D2Y, V9G, V6B
Hand #17: V5R, V8Y, V2B, C-Y, V4Y, V8R, V3Y
Hand #18: V8B, *-*, D4*, V6R, C-G, V9Y, V1B
Hand #19: V4B, *-*, V0B, D2B, V3R, *-*, V0Y
Hand #20: R-R, V4G, C-R, V5B, V6G, R-G, V7R
Hand #21: D2Y, V5Y, V5Y, V4B, V3G, V2G, V3R
Hand #22: V1R, V4R, V4G, *-*, V7R, D2Y, D2R
Hand #23: R-G, V6G, R-R, V7R, D2R, V3G, *-*
Hand #24: V0Y, C-G, *-*, V9G, V1R, C-B, V5R
Hand #25: V9Y, V5Y, V5B, V9Y, V4R, V1R, V7G
Hand #26: V4G, V8R, R-G, C-B, V3B, V2R, V1Y
Hand #27: C-Y, V4G, D2R, *-*, V5Y, D2B, R-B
Hand #28: V5G, V2Y, V6B, V1R, V3B, V9Y, V1R
Hand #29: V1R, D4*, V7G, V9Y, V6B, V2Y, V3B
Hand #30: V0Y, V0R, D4*, V9Y, V3G, R-B, V4R
Hand #31: V2G, V0B, D2R, V6R, R-G, D2G, V3Y

0: H0R, 1: H0Y, 2: H0B, 3: H0G, 4: H2R, 5: H2Y, 6: H2B, 7: H2G, 8: D4->R, 9: D4->Y, 10: D4->B, 11: D4->G
```

## Future Goals

* Add different AI tactics for analysis.
* Supply graphical analysis of plays.
