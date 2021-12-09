# Bloxide

## Overview

Bloxide is a Rust implementation of a classic snake arcade game.  In this game, the two players move around the screen leaving permanent trails in their wake.  The goal is to make your opponent crash into a wall, your trail, or its own trail before you do so.  In this version of the game, the two players start at opposite corners of the screen.

## Instructions for Use

To run the program, clone the git repository and then run the command `cargo run`.  

By default, both players are controlled by the keyboard.  Player 1 (red) is controlled by the WASD keys, and player 2 (blue) is controlled by the arrow keys.  If the 'p' key is pressed during the game, this will toggle the AI on and off.  Visually you can tell when the AI is activated because player 1's trail becomes green instead of red, and it is no longer able to be controlled by the arrow keys unless it is toggled again.  

## AI

The AI for this game is straightforward - it mainly tries to take as wide of a loop around the game as possible, in the hopes that the other player will make a mistake when trying to cut it off.  
