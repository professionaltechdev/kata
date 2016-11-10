# Langton's Ant

https://en.wikipedia.org/wiki/Langton%27s_ant

Langton's ant is a two-dimensional Turing machine with a very simple set of
rules but complex emergent behavior. It was invented by Chris Langton in 1986
and runs on a square lattice of black and white cells.

In this kata we'll create a two-dimensional grid, which will start entirely
white, and then put an ant on it which will move according to a very simple
set of rules:

* At a white square, turn 90° right, flip the color of the square, move forward one unit
* At a black square, turn 90° left, flip the color of the square, move forward one unit

We'll write a program that steps through this simple simulation to draw out
patterns.


# Tasks

* Build a 2 dimensional array to represent a grid. Make it 11x11 to start
with - all cells should be white (you can represent a cell with a boolean if
you like)
* Draw the grid (just to stdout, if that's easiest)
* Add an ant at grid co-ordinate 5,5, facing East
* The ant is on a white square to begin, so follow the first rule: turn the
ant 90° right, flip the square to black, and move the and forward one unit
* Draw the new grid
* Repeat these two steps 4 times. Your ant should now be on a black square
* Write the rules for black squares and have your ant follow them
* Continue to follow the rules to produce new grids each time. Solve the 200th
iteration of the algorithm - at this point the patterns will be simple and
largely symmetrical
* Try running the pattern up to a few thousand steps to see some chaos
* Try going past 10,000 steps to see your little ant form a highway


# Bonus

* Add a third colour. With three colours, the colours are modified in a cyclic
fashion. Whether the ant turns right or left on each colour is up to you!
* Add a fourth colour. If we denote left and right turns as L and R, and the
original Langton's ant is of the form "RL", try the form "LLRR"
* Improve the display of your program. Perhaps try a live-updating JS canvas,
or a cool console library

Got an idea for a bonus task? Fork the repo and add it!


# Submitting an entry for this kata

All languages are welcome. This kata exists purely to exercise your brains -
don't feel obliged to write concise code, or lengthy code, or clever code. Just
do whatever makes you happy and stretches your brain.

If you'd like people to review your code, you are very welcome to fork this repo
and open a pull request, adding the code into a subfolder beneath this one
with the same name as your github username. For example, I'm `veryhappythings`,
so I'd put my code in `kata2/veryhappythings`.
