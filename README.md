# Uno.rs

A Uno simulator written in Rust. Used to determining Uno optimal strategies.

## How to Run

If running from the repo, you can use the default cargo manager:

```
cargo run --release -- --help
Usage: uno.exe [OPTIONS]

Options:
  -p, --players <PLAYERS>      Number of players. [default: 2]
  -n, --num-decks <NUM_DECKS>  Number of uno decks to play with, there are 108 cards per game. [default: 1]
  -x, --hand-size <HAND_SIZE>  Hand size during the game. [default: 7]
  -s, --seed <SEED>            Seed to play the game. [default: 0]
      --play-as <PLAY_AS>      Select a player to play as otherwise the game is simulated. [default: -1]
  -h, --help                   Print help
  -V, --version                Print version
```

Default behavior simulates a game with random input. Use `--play-as` to play as a player.

```
cargo run --release -- --hand-size 2 --play-as 0

--- Playing as Hand #0 ---
Field Top: V1Y
Cards in Deck: 103
Cards in Field: 1

Hand #0: *-*, V2R
Hand #1: V5Y, V2R

0: *-R 1: *-Y 2: *-B 3: *-G

Input a valid move from 0 - 3
0

--- Playing as Hand #1 ---
Field Top: *-R
Cards in Deck: 103
Cards in Field: 2

Hand #0: V2R
Hand #1: V5Y, V2R

0: V2R

Bot has picked move: 0

--- Playing as Hand #0 ---
Field Top: V2R
Cards in Deck: 103
Cards in Field: 3

Hand #0: V2R
Hand #1: V5Y

0: V2R

Input a valid move from 0 - 0
0

Player 0 has won the game!
```

## Interpretting the IR

To keep the game succinct in terminal, an IR is used for display cards.

A general notation is as follows:

* `<Card Type><Card Value><Card Color>`

Each card can be encoded using this approach for example:

* Value Card: `V<Value><Color>`
* Draw Card: `D<Number_Cards><Color Chosen>`

Generally dashes denote non-value or irrelevant value.
Here are some examples of the IR:

* Wild Card: `*-`
* Wild Card Played with Color: `*-R`
* Draw Four without Color Selected: `D4*`
* Draw Four with Color Selected: `D4R`

## Custom Uno Games

The script allows for tweaking of common configuration parameters.
Upon specifying `--players` for example, number of decks being used will scale.
Here is an example of a 24 player game:

```
$> cargo run -- --players 24
Number of players exceed number of cards available if each player has 7 cards per hand.
Auto scaling num of decks to 2 for 24 players.
--- Playing as Hand #0 ---
Field Top: V2Y
Cards in Deck: 47
Cards in Field: 1

Hand #0: D4*, V3R, V7R, D4*, R-B, V4R, V4Y
Hand #1: V1G, V4B, V1B, D4*, R-G, V7R, V7G
Hand #2: D2R, V6R, S-B, V5Y, *-*, V7R, D2R
Hand #3: S-B, V3B, V5G, R-B, V8R, R-B, V0B
Hand #4: V5G, R-R, S-G, V0G, V4B, *-*, V8Y
Hand #5: S-B, V3R, V6Y, V3B, D2G, V9R, V1G
Hand #6: V8R, V5R, V6Y, V1Y, V8B, V3Y, R-R
Hand #7: V7R, V0R, D2R, V1Y, V1R, V7B, V6G
Hand #8: V3B, V8Y, V7B, R-G, V4G, V1B, V2B
Hand #9: V7G, *-*, V7Y, R-G, D2G, D4*, V5R
Hand #10: *-*, R-Y, V6B, V6B, V1R, V4G, D4*
Hand #11: V2R, V8Y, V3G, V7G, R-Y, D2Y, V8B
Hand #12: R-B, V6R, V1Y, S-R, V2G, V3R, D2G
Hand #13: D2Y, V7G, D2Y, V5Y, V6R, V9Y, V8Y
Hand #14: V2G, *-*, V5B, V9G, V0R, *-*, V4G
Hand #15: V2Y, S-R, R-Y, S-R, D2B, V0B, V1G
Hand #16: V4R, V6G, D4*, V9B, V6G, S-Y, D4*
Hand #17: V1B, V7Y, V5Y, V3R, S-Y, V4G, V2Y
Hand #18: V6Y, V0G, V3G, V8R, V2B, V3G, V9B
Hand #19: R-G, V9Y, V6B, V2G, V2R, V4R, *-*
Hand #20: R-R, V5G, S-Y, S-G, V3Y, R-R, S-Y
Hand #21: D2R, V4Y, V6B, V9Y, V0Y, V6Y, V2R
Hand #22: V8R, V5Y, V2B, S-R, V9G, V9R, V5B
Hand #23: V8B, V9R, V4R, D4*, V1R, V5B, D2B

0: D4->R 1: D4->Y 2: D4->B 3: D4->G 4: D4->R 5: D4->Y 6: D4->B 7: D4->G 8: V4Y
```

## Future Goals

* Add different AI tactics for analysis.
* Supply graphical analysis of plays.
