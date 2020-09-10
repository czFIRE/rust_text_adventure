# rust_text_adventure
Implementation of text adventure in rust

## Format for the text of the adventure:
\#Format: "name" "modifier" ("location of mosters" if it is a fight scene)
\#        "text"
\#        "choices"
\#-------------------------

## Format for the monster scene:
\#Format: "player life" "player light attack damage" "player heavy attack damage"
\#        "monster name" "monster life" "monster damage_max"
\#        ...

Player can either choose to do 2 light swings or 1 heavy swing.
Heavy swing deals 1 damage guaranteed, whilst light swing can miss and deal 0 damage.

Currently implemented games are:
txt_adventure.txt
kidnapper1.txt

You are expected to start the game using:
cargo run txt_adventure.txt
(or you can just use the binary file and provide it with the correct file)