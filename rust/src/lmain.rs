mod maze;
use godot::builtin::Vector2i;
use maze::*;
use rand::rng;


fn main() {
    let super_maze = SuperMaze::new();
    println!("{}", super_maze.structural_maze);
    let maze = super_maze.mazes[0][0];
    println!("{}", maze);
    println!("{:?}", super_maze.structural_maze.get_walls_for_cell(Vector2i{x:0,y:0}));
}
