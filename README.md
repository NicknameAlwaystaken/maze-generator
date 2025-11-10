Maze generation using similar to Loop-erased random walk (also known as Wilson's Algorithm).
I create random walks, first walk needs to be long enough before accepted, then additional walks have to hit original path without looping into itself, otherwise erased.

So in short, I don't do efficient loop cutting currently, I just discard the whole walk, I may implement it sometime in the future.

Image of complete maze with solution.
![](docs/completed_maze.png)

Small gif of generation of maze.
![](docs/demo.gif)

