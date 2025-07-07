use std::fmt::{Display, Formatter, Result};
use std::collections::HashSet;
use std::slice::GetDisjointMutError;
use godot::obj::Gd;
use godot::prelude::{godot_api, Export, GodotClass, GodotConvert, Var};
use rand::seq::{IndexedRandom, IteratorRandom};
use rand::{rng, Rng};
use rand::rngs::ThreadRng;
use godot::builtin::Vector2i;

const MAZE_SIZE:usize = 5;


macro_rules! gvec {
    ($x:expr, $y:expr) => {
        Vector2i{x:$x as i32, y:$y as i32}
    };
}

const RIGHT_VEC: Vector2i = gvec!(1,0);
const LEFT_VEC: Vector2i = gvec!(-1,0);
const DOWN_VEC: Vector2i = gvec!(0,1);
const UP_VEC: Vector2i = gvec!(0,-1);


const TOP_EXIT: Vector2i = gvec!(MAZE_SIZE/2+1, 0);
const LEFT_EXIT: Vector2i = gvec!(0, MAZE_SIZE/2+1);
const RIGHT_EXIT: Vector2i = gvec!(MAZE_SIZE-1, MAZE_SIZE/2+1);
const ENTRANCE: Vector2i = gvec!(MAZE_SIZE/2+1, MAZE_SIZE-1);

pub fn gen_hashset(size: usize) -> HashSet<Vector2i> {
    let mut set= HashSet::new();
    for x in 0..size {
        for y in 0..size {
            set.insert(gvec!(x as isize,y as isize));
        }
    }
    set
}


trait Coord<T> { 
    fn gc(&self, coords: &Vector2i) -> &T;
    fn gc_mut(&mut self, coords: &Vector2i)  -> &mut T;
}

impl<T, const N: usize, const M: usize> Coord<T> for [[T; N]; M] {

    fn gc(&self, coords: &Vector2i)  -> &T {
        &self[coords.y as usize][coords.x as usize]
    }

    fn gc_mut(&mut self, coords: &Vector2i)  -> &mut T {
        &mut self[coords.y as usize][coords.x as usize]
    }
}



#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    None,
    Up,
    Right,
    Down, 
    Left,
}

impl Direction {
    fn possible_directions(&self, pos: &Vector2i) -> Vec<Direction> {
        let mut directions= Vec::new();
        let outer_index =  (MAZE_SIZE-1) as i32;
        if *self != Direction::Right && pos.x != 0 {
            directions.push(Direction::Left);
        }
        if *self != Direction::Down &&pos.y != 0 {
            directions.push(Direction::Up);
        }
        if *self != Direction::Left &&pos.x != outer_index {
            directions.push(Direction::Right);
        }
        if *self != Direction::Up &&pos.y != outer_index {
            directions.push(Direction::Down);
        }
        directions
            
    }

    fn rnd_possible_next(&self, rng: &mut ThreadRng, pos: &Vector2i) -> Direction {
        *self.possible_directions(pos).choose(rng).unwrap()
    }

    fn offset(self) -> Vector2i {
        match self {
            Direction::Up => gvec!(0, -1),
            Direction::Right => gvec!(1, 0),
            Direction::Down => gvec!(0, 1),
            Direction::Left => gvec!(-1, 0),
            Direction::None => panic!("Should never be called on Direction None")
        }
    }

    fn s_wall_offset(self) -> Vector2i {
        match self {
            Direction::Up => gvec!(0, -1),
            Direction::Left => gvec!(-1, 0),
            _ => gvec!(0,0)
        }
    }
    fn b_wall_offset(self) -> Vector2i {
        match self {
            Direction::Down => gvec!(0, 1),
            Direction::Right => gvec!(1, 0),
            _ => gvec!(0,0)
        }
    }

    fn is_vertical(&self) -> bool {
        *self == Direction::Up || *self == Direction::Down
    }

    fn prio_from_current(&self, reversed: bool) -> [Direction; 4] {
        let mut prio_list = match self {
            Direction::Up => [Direction::Left, Direction::Up, Direction::Right, Direction::Down],
            Direction::Right => [Direction::Up, Direction::Right, Direction::Down, Direction::Left],
            Direction::Down => [Direction::Right, Direction::Down, Direction::Left, Direction::Up,],
            Direction::Left => [Direction::Down, Direction::Left, Direction::Up, Direction::Right],
            Direction::None => panic!("Should never be called on Direction None")
        };
        if reversed{
            prio_list.reverse();
        }
        prio_list
    }
}

impl From<Vector2i> for Direction {
    fn from(value: Vector2i) -> Self {
        match value {
            RIGHT_VEC => Direction::Right,
            LEFT_VEC => Direction::Left,
            DOWN_VEC => Direction::Down,
            UP_VEC => Direction::Up,
            _ => panic!("Not a valid Input")
        }
    }
} 

pub struct MazeGenerator {
    cells: [[bool; MAZE_SIZE]; MAZE_SIZE],
    vertical_walls: [[bool; MAZE_SIZE]; MAZE_SIZE-1],
    horizontal_walls: [[bool; MAZE_SIZE-1]; MAZE_SIZE],
    unvisited: HashSet<Vector2i>

}

impl MazeGenerator {
    pub fn new(rng: &mut ThreadRng) -> MazeGenerator {
        let mut maze = MazeGenerator { 
            cells: [[false; MAZE_SIZE]; MAZE_SIZE], 
            vertical_walls: [[true; MAZE_SIZE]; MAZE_SIZE-1],
            horizontal_walls: [[true; MAZE_SIZE-1]; MAZE_SIZE],
            unvisited: gen_hashset(MAZE_SIZE),
        };
        maze.generate_maze(rng);
        maze
    }

    fn generate_maze(&mut self, rng: &mut ThreadRng) {

        // visit first cell
        self.visit_cell(&gvec!(
            rng.random_range(..MAZE_SIZE) as isize,
            rng.random_range(..MAZE_SIZE) as isize
        ));
        
        //Main loop
        while !self.unvisited.is_empty() {
            let mut current = *self.unvisited.iter().choose(rng).unwrap();
            let mut path: Vec<Vector2i> = vec![current];
            let mut dir_path: Vec<Direction> = Vec::new();

            let mut direction = Direction::None.rnd_possible_next(rng, &current);
            // while the current cell is not visited
            while !self.cells.gc(&current) {
                // select next cell
                direction = direction.rnd_possible_next(rng, &current);
                let next = current + direction.offset();

                // kill loops 
                if let Some(pos) = path.iter().position(| &p | p == next) {
                    path.truncate(pos + 1);
                    dir_path.truncate(pos);
                } else {
                    path.push(next);
                    dir_path.push(direction);
                }

                current = next;
            }
            for (index, dir) in dir_path.into_iter().enumerate() {
                let coords = &path[index];
                self.visit_cell(coords);
                self.remove_wall(coords, dir);
            }
            self.visit_cell(path.last().unwrap());
        }

    }

    fn visit_cell(&mut self, coords: &Vector2i) {
        self.unvisited.remove(coords);
        *self.cells.gc_mut(coords) = true;
    }

    fn remove_wall(&mut self, coords: &Vector2i, direction:Direction) {
        let calculated_coords = *coords + direction.s_wall_offset();
        if direction.is_vertical() {
            *self.vertical_walls.gc_mut(&calculated_coords) = false
        } else {
            *self.horizontal_walls.gc_mut(&calculated_coords) = false
        }
    }
    pub fn extract_maze(&self) -> Maze {
        let mut vertical_walls = [[true; MAZE_SIZE]; MAZE_SIZE+1];
        for y in 0..MAZE_SIZE-1 {
            for x in 0..MAZE_SIZE {
                vertical_walls[y+1][x] = self.vertical_walls[y][x];
            }
        }
        let mut horizontal_walls = [[true; MAZE_SIZE+1]; MAZE_SIZE];
        for y in 0..MAZE_SIZE {
            for x in 0..MAZE_SIZE-1 {
                horizontal_walls[y][x+1] = self.horizontal_walls[y][x];
            }
        }

        Maze {
            vertical_walls,
            horizontal_walls,
            cells: [[gvec!(0,0); MAZE_SIZE]; MAZE_SIZE]
        }
    }

}

impl Display for MazeGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let max = MAZE_SIZE-1;
        writeln!(f)?;
        write!(f, " {} ", "_ ".repeat(MAZE_SIZE))?;
        writeln!(f)?;
        for row_index in 0..max {
            write!(f,"|")?;
            for val_index in 0..max {
                let coord = gvec!(val_index, row_index);
                if *self.vertical_walls.gc(&coord) {
                    write!(f,"_")?;
                } else {
                    write!(f," ")?;
                }
                if *self.horizontal_walls.gc(&coord) {
                    write!(f,"|")?;
                } else {
                    write!(f," ")?;
                }
                
            }
            if *self.vertical_walls.gc(&(gvec!(max,row_index))){
                write!(f,"_")?;
            } else {
                write!(f," ")?;
            }
            write!(f,"|")?;
            writeln!(f)?;
        }
        write!(f,"|")?;
        for val in self.horizontal_walls[max] {
            write!(f,"_")?;
            if val {
                write!(f,"|")?;
            } else {
                write!(f," ")?;
            }
        }
        write!(f,"_|")?;
        Ok(())
    }

}

#[derive(PartialEq, Eq, Hash, GodotClass, Clone, Copy)]
#[class(no_init,base=RefCounted)]
pub struct Maze {
    vertical_walls: [[bool; MAZE_SIZE]; MAZE_SIZE+1],
    horizontal_walls: [[bool; MAZE_SIZE+1]; MAZE_SIZE],
    pub cells: [[Vector2i; MAZE_SIZE]; MAZE_SIZE],
}
#[godot_api]
impl Maze {

    pub fn new() -> Maze {
        let mut rng = rng();
        let mut maze_gen = MazeGenerator::new(&mut rng);
        maze_gen.generate_maze(&mut rng);
        maze_gen.extract_maze()
        
    }

    fn gen_new_maze(connector_walls: (bool, bool, bool, bool)) -> Maze {
        let mut new_maze = Maze::new();
        let (left, up , right, down) = connector_walls;
        new_maze.set_outer(left, up, right, down);
        new_maze.build();
        new_maze
    }

    pub fn set_outer(&mut self, left: bool, up: bool, right: bool,down: bool) {

        self.horizontal_walls[MAZE_SIZE/2][0] = left;
        self.vertical_walls[0][MAZE_SIZE/2] = up;
        self.horizontal_walls[MAZE_SIZE/2][MAZE_SIZE] = right;
        self.vertical_walls[MAZE_SIZE][MAZE_SIZE/2] = down;

    }

    pub fn get_walls_for_cell(&self, v: Vector2i) -> (bool, bool, bool, bool) {
        (
            self.get_wall(v, Direction::Left),
            self.get_wall(v, Direction::Up),
            self.get_wall(v, Direction::Right),
            self.get_wall(v, Direction::Down)
        )
       
    }

    pub fn build(&mut self){
        for y in 0..MAZE_SIZE {
            for x in 0..MAZE_SIZE {
                let walls = self.get_walls_for_cell(gvec!(x,y));
                self.cells[y][x] = match walls {
                    (true, false, false, false) => gvec!(0,1),
                    (false, true, false, false) => gvec!(1,0),
                    (false, false, true, false) => gvec!(2,1),
                    (false, false, false, true) => gvec!(1,2),

                    (true, true, false, false) => gvec!(0,0),
                    (false, true, true, false) => gvec!(2,0),
                    (false, false, true, true) => gvec!(2,2),
                    (true, false, false, true) => gvec!(0,2),

                    (true, false, true, false) => gvec!(3,0),
                    (false, true, false, true) => gvec!(3,2),

                    (false, true, true, true) => gvec!(5,1),
                    (true, false, true, true) => gvec!(4,2),
                    (true, true, false, true) => gvec!(3,1),
                    (true, true, true, false) => gvec!(4,0),

                    (false, false, false, false) => gvec!(1,1),
                    (true, true, true, true) => gvec!(4,1),
                }
            }
        }
        
    }


    #[func]
    pub fn get(&self, v: Vector2i) -> Vector2i {
        *self.cells.gc(&v)
    }

    #[func]
    pub fn get_wall_g(&self,coords: Vector2i, direction: Vector2i) -> bool {
        self.get_wall(coords, Direction::from(direction))
    }

    pub fn get_wall(&self,coords: Vector2i, direction: Direction) -> bool {
        let calculated_coords = coords + direction.b_wall_offset();
        if direction.is_vertical() {
            *self.vertical_walls.gc(&calculated_coords)
        } else {
            *self.horizontal_walls.gc(&calculated_coords)
        }
    }

    


    fn possible_directions(&self, coords: &Vector2i) -> Vec<Direction> {
        let mut dirs = Vec::new();

        if coords.y != 0 && !self.vertical_walls.gc(coords) {
            dirs.push(Direction::Up);
        }

        if coords.x != 0 && !self.horizontal_walls.gc(coords) {
            dirs.push(Direction::Left);
        }

        let max_index = MAZE_SIZE as i32 - 1;
        if coords.y != max_index && !self.vertical_walls.gc(coords)  {
            dirs.push(Direction::Down);
        }
        if coords.x != max_index && !self.horizontal_walls.gc(coords) {
            dirs.push(Direction::Right);
        }

        dirs
    }

}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        //first row
        writeln!(f)?;
        write!(f," ")?;
        for val in self.vertical_walls[0] {
            if val {
                write!(f,"_ ")?;
            } else {
                write!(f,"  ")?;
            }
        }
        for y in 0..MAZE_SIZE {
            writeln!(f)?;
            for x in 0..MAZE_SIZE {
                let coord = gvec!(x, y);
                if *self.horizontal_walls.gc(&coord) {
                    write!(f,"|")?;
                } else {
                    write!(f," ")?;
                }
                let coord = gvec!(x, y+1);
                if *self.vertical_walls.gc(&coord) {
                    write!(f,"_")?;
                } else {
                    write!(f," ")?;
                }
            }
            let coord = gvec!(MAZE_SIZE, y);
            if *self.horizontal_walls.gc(&coord) {
                write!(f,"|")?;
            } else {
                write!(f," ")?;
            }
        }
        Ok(())
    }

}


#[derive(GodotClass)]
#[class(no_init,base=RefCounted)]
pub struct SuperMaze {
    pub structural_maze: Maze,
    pub mazes: [[Maze; MAZE_SIZE]; MAZE_SIZE],
}

#[godot_api]
impl SuperMaze {

    pub fn new () -> SuperMaze {
        let mut structural_maze = Maze::new();

        structural_maze.set_outer(true, false, true, false);

        let mut mazes = [[
            Maze {
                vertical_walls: [[false; MAZE_SIZE]; MAZE_SIZE+1],
                horizontal_walls: [[false; MAZE_SIZE+1]; MAZE_SIZE],
                cells: [[gvec![0,0]; MAZE_SIZE]; MAZE_SIZE]
            };MAZE_SIZE]; MAZE_SIZE];

        for y in 0..MAZE_SIZE {
            for x in 0..MAZE_SIZE {
                mazes[y][x] = Maze::gen_new_maze(structural_maze.get_walls_for_cell(gvec!(x,y)));
            }
        }

        SuperMaze { 
            structural_maze,
            mazes
        }
    }

    #[func]
    pub fn create() -> Gd<SuperMaze> {

        Gd::from_init_fn(|_| SuperMaze::new())
    }

    #[func]
    pub fn reload(&mut self, v: Vector2i) {
        *self.mazes.gc_mut(&v) = Maze::gen_new_maze(self.structural_maze.get_walls_for_cell(v));
    }

    #[func]
    pub fn retrieve_maze(&self, v: Vector2i) -> Gd<Maze>{
        Gd::from_object(self.mazes.gc(&v).clone())
    }

}