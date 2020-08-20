use rand::seq::SliceRandom;

// Mask managing the maze's tiles' walls
const NORTH: u8 = 0b01000;
const SOUTH: u8 = 0b00100;
const WEST: u8 = 0b00010;
const EAST: u8 = 0b00001;
const FULL: u8 = NORTH | SOUTH | WEST | EAST;
// Mask checking if a tile was visited during the maze generation
const VISITED: u8 = 0b10000;

pub struct Maze {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl Maze {
    /// Constructor
    /// # Arguments
    /// * `width` - Maze's width
    /// * `height` - Maze's height
    pub fn new(width: usize, height: usize) -> Maze {
        Maze {
            grid: vec![FULL; width * height],
            width: width,
            height: height,
        }
    }
    /// Starting function for the recursive backtracker algorithm
    pub fn recursive_backtracker(&mut self) {
        self.tile_visit(0, 0);
    }
    pub fn has_north(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y) & NORTH != 0
    }
    pub fn has_south(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y) & SOUTH != 0
    }
    pub fn has_west(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y) & WEST != 0
    }
    pub fn has_east(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y) & EAST != 0
    }
    /// Return the tile at (x;y)
    /// #Arguments
    /// * `x` - Tile's abscissa
    /// * `y` - Tile's ordinate
    pub fn get_tile(&self, x: usize, y: usize) -> u8 {
        self.grid[self.width * y + x]
    }
    /// Returns the maze's width
    pub fn get_width(&self) -> usize {
        self.width
    }
    /// Returns the maze's height
    pub fn get_height(&self) -> usize {
        self.height
    }
    /// Check if a tile is not visited
    /// #Arguments
    /// * `x` - Tile's abscissa
    /// * `y` - Tile's ordinate
    fn is_not_visited(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y) & VISITED == 0
    }
    /// Break one of the four wall of a tile
    /// #Arguments
    /// * `x` - Tile's abscissa
    /// * `y` - Tile's ordinate
    /// * `wall` - Wall to break (SOUTH, NORTH, EAST, WEST)
    fn break_wall(&mut self, x: usize, y: usize, wall: u8) {
        self.grid[self.width * y + x] ^= wall;
    }
    /// Recursive function to make the maze
    /// #Arguments
    /// * `x` - Tile's abscissa
    /// * `y` - Tile's ordinate
    fn tile_visit(&mut self, x: usize, y: usize) {
        self.grid[self.width * y + x] |= VISITED;
        //println!("{}, {}", x, y);
        let choices = [NORTH, SOUTH, WEST, EAST];
        let shuffled = choices.choose_multiple(&mut rand::thread_rng(), 4);
        /*
         * It will try every direction, if a direction has not been visited
         * it will visit this direction and call this recursive function
         * on the non visited tile
         */
        for dir in shuffled {
            match *dir {
                NORTH => {
                    // We check if the tile at north is not out of bound
                    // and if this tile was not visited
                    if y > 0 && self.is_not_visited(x, y-1) {
                        //We break the walls to link the two tiles
                        self.break_wall(x, y, NORTH);
                        self.break_wall(x, y-1, SOUTH);
                        self.tile_visit(x, y-1);
                    }
                }
                SOUTH => {
                    if y < self.height - 1 && self.is_not_visited(x, y+1) {
                        self.break_wall(x, y, SOUTH);
                        self.break_wall(x, y+1, NORTH);
                        self.tile_visit(x, y+1);
                    }
                }
                WEST => {
                    if x > 0 && self.is_not_visited(x-1, y) {
                        self.break_wall(x, y, WEST);
                        self.break_wall(x-1, y, EAST);
                        self.tile_visit(x-1, y);
                    }
                }
                EAST => {
                    if x < self.width -1 && self.is_not_visited(x+1, y) {
                        self.break_wall(x, y, EAST);
                        self.break_wall(x+1, y, WEST);
                        self.tile_visit(x+1, y);
                    }
                }
                _ => panic!("Unknown direction provided after shuffling."),
            }
        }
    }
}