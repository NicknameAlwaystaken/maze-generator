use std::thread::sleep;
use std::time::Duration;
use rand::seq::SliceRandom;
use rand::Rng;
use colored::*;

fn main() {
    let mut maze = Maze::new(151, 51);
    maze.generate_maze();
    maze.print();
    maze.find_path();
    //maze.print();
}

#[derive(Clone, Copy)]
struct Cell {
    visited: bool,
}

impl Cell {
    fn new() -> Self {
        Cell {
            visited: false,
        }
    }
}

#[derive(Clone, Debug)]
struct Path {
    x: usize,
    y: usize,
    previous: Option<Box<Path>>,
    visited: bool,
}

impl Path {
    fn new(x: usize, y: usize) -> Self {
        Path {
            x,
            y,
            previous: None,
            visited: false,
        }
    }
}

struct Maze {
    grid: Vec<Vec<char>>,
    cells: Vec<Vec<Cell>>,
    paths: Vec<Vec<Path>>,
    width: usize,
    height: usize,
}

impl Maze {
    fn new(new_width: usize, new_height: usize) -> Self {
        let mut width = new_width;
        let mut height = new_height;
        if width % 2 == 0 {
            width += 1
        }
        if height % 2 == 0 {
            height += 1
        }
        let mut cells = vec![vec![Cell::new(); width]; height];
        let grid = {
            let mut vector: Vec<Vec<char>> = Vec::new();
            for y in 0..height {
                let mut vectorx: Vec<char> = Vec::new();
                for x in 0..width {
                    if x % 2 == 1 || y % 2 == 1 {
                        cells[y][x].visited = true;
                    }
                    if x % 2 == 0 && y % 2 == 0 {
                        vectorx.push('.');
                        continue;
                    }
                    vectorx.push('#');
                }
                vector.push(vectorx)
            }
            vector
        };
        let paths = {
            let mut vector: Vec<Vec<Path>> = Vec::new();
            for y in 0..height {
                let mut vectorx: Vec<Path> = Vec::new();
                for x in 0..width {
                    vectorx.push(Path::new(x,  y));
                }
                vector.push(vectorx)
            }
            vector
        };

        Self {
            grid,
            cells,
            paths,
            width,
            height
        }
    }

    fn reset_grid(&mut self) {
        self.cells = vec![vec![Cell::new(); self.width]; self.height];
        self.grid = {
            let mut vector: Vec<Vec<char>> = Vec::new();
            for y in 0..self.height {
                let mut vectorx: Vec<char> = Vec::new();
                for x in 0..self.width {
                    if x % 2 == 1 || y % 2 == 1 {
                        self.cells[y][x].visited = true;
                    }
                    if x % 2 == 0 && y % 2 == 0 {
                        vectorx.push('.');
                        continue;
                    }
                    vectorx.push('#');
                }
                vector.push(vectorx)
            }
            vector
        };
    }

    fn find_path(&mut self) {
        let x: i32 = 0;
        let y: i32 = 0;

        fn path(maze: &mut Maze, x: i32, y: i32) -> Option<Path> {
            maze.paths[y as usize][x as usize].visited = true;
            //maze.print();
            //println!("Pos:{},{}", x, y);
            //sleep(Duration::from_secs(1));
            let moves: [(i32, i32); 4] = [(2, 0), (0, 2), (-2, 0), (0, -2)];
            if x == maze.width as i32 - 1 && y == maze.height as i32 - 1 {
                return Some(maze.paths[y as usize][x as usize].clone());
            }
            for (xi, yj) in moves {
                if x as i32 + xi >= 0 as i32 && x as i32 + xi < maze.width as i32 &&
                    y as i32 + yj >= 0 as i32 && y as i32 + yj < maze.height as i32 {
                    if maze.paths[(y + yj) as usize][(x + xi) as usize].visited {
                        continue;
                    }
                    if maze.grid[(y + yj / 2) as usize][(x + xi / 2) as usize] == '#' {
                        continue;
                    }
                    maze.paths[(y + yj) as usize][(x + xi) as usize].previous = Some(Box::new(maze.paths[y as usize][x as usize].clone()));
                    if let Some(found_path) = path(maze, x + xi , y + yj) {
                        return Some(found_path);
                    }
                }
            }
            None
        }

        if let Some(found_path) = path(self, x , y) {
            for y in 0..self.height {
                for x in 0..self.width {
                    self.paths[y][x].visited = false;
                }
            }
            let mut counter = 0;
            let mut current_path: Option<&Path> = Some(&found_path);
            self.paths[current_path.unwrap().y][current_path.unwrap().x].visited = true;
            while let Some(path) = current_path {
                counter += 1;
                let prev_path = path.previous.as_deref();
                if let Some(prev) = prev_path{
                    if path.x == prev.x && (path.y as isize - prev.y as isize).abs() == 2 {
                        let mid_y = (path.y + prev.y) / 2;
                        self.paths[mid_y][path.x].visited = true;
                    } else if path.y == prev.y && (path.x as isize - prev.x as isize).abs() == 2 {
                        let mid_x = (path.x + prev.x) / 2;
                        self.paths[path.y][mid_x].visited = true;
                    }

                    self.paths[prev.y][prev.x].visited = true;
                }

                current_path = prev_path;
            }
            self.print();
            println!("Maze solved! Steps required: {}", counter);
        }
        else {
            self.print();
            println!("Maze was not solved!")
        }
    }

    fn generate_maze(&mut self) {
        let mut rng = rand::thread_rng();

        let mut cell_x: i32 = rng.gen_range((0 / 2) as i32..(self.width / 2) as i32) * 2;
        let mut cell_y: i32 = rng.gen_range((0 / 2) as i32..(self.height / 2) as i32) * 2;

        self.cells[cell_y as usize][cell_x as usize].visited = true;
        self.grid[cell_y as usize][cell_x as usize] = '.';

        let mut remaining_cells: Vec<(i32, i32)> = Vec::new();
        for i in 0..self.height {
            for j in 0..self.width {
                if !self.cells[i][j].visited {
                    remaining_cells.push((j as i32, i as i32));
                }
            }
        }
        //println!("Remaining cells: {}", remaining_cells.len());

        let moves: [(i32, i32); 4] = [(2, 0), (0, 2), (-2, 0), (0, -2)];
        let mut prev_move_x = -1;
        let mut prev_move_y = -1;
        let mut current_path_steps: Vec<(i32, i32)> = Vec::new();
        current_path_steps.push((cell_x, cell_y));
        let mut first_walk = true;
        let mut erase_path = false;
        loop {
            loop {
                let (mut random_x, mut random_y) = moves.choose(&mut rng).unwrap();
                loop {
                    if random_x == -prev_move_x && random_y == -prev_move_y {
                        (random_x, random_y) = *moves.choose(&mut rng).unwrap();
                    } else {
                        break;
                    }
                }
                let x = cell_x + random_x;
                let y = cell_y + random_y;
                //println!("Prev Cell: {},{} Cell: {},{} move: {},{}", prev_move_x, prev_move_y, cell_x, cell_y, random_x, random_y);

                if x >= 0 && x < self.width as i32 &&
                y >= 0 && y < self.height as i32 {
                    for (xi, yj) in current_path_steps.iter() {
                        if x == *xi && y == *yj {
                            erase_path = true;
                        }
                    }
                    prev_move_x = random_x;
                    prev_move_y = random_y;
                    cell_x = x;
                    cell_y = y;
                    self.grid[(cell_y - random_y / 2) as usize][(cell_x - random_x / 2) as usize] = '.';
                    current_path_steps.push((x, y));
                    current_path_steps.push(((cell_x - random_x / 2), (cell_y - random_y / 2)));
                    break;
                }
            }
            if !self.cells[cell_y as usize][cell_x as usize].visited {
                self.cells[cell_y as usize][cell_x as usize].visited = true;
                remaining_cells = remaining_cells.into_iter().filter(|(xi, yj)| !self.cells[*yj as usize][*xi as usize].visited).collect::<Vec<(i32, i32)>>();
                //println!("Continuing on cell: {},{}", cell_x, cell_y);
            }
            else {
                if first_walk && current_path_steps.len() / 2 >= 20  {
                    erase_path = false;
                    first_walk = false;
                }
                else if first_walk && erase_path{
                    cell_x = rng.gen_range((0 / 2) as i32..(self.width / 2) as i32) * 2;
                    cell_y = rng.gen_range((0 / 2) as i32..(self.height / 2) as i32) * 2;
                    prev_move_x = 0;
                    prev_move_y = 0;
                    current_path_steps = Vec::new();
                    self.reset_grid();
                    remaining_cells = Vec::new();
                    for i in 0..self.height {
                        for j in 0..self.width {
                            if !self.cells[i][j].visited {
                                remaining_cells.push((j as i32, i as i32));
                            }
                        }
                    }
                    erase_path = false;
                    continue;
                }
                else if erase_path {
                    //println!("Erasing path!");
                    for (xi, yj) in current_path_steps.iter() {
                        if *xi % 2 == 1 || *yj % 2 == 1 {
                            self.grid[*yj as usize][*xi as usize] = '#';
                        } else {
                            self.cells[*yj as usize][*xi as usize].visited = false;
                        }
                    }
                    remaining_cells = Vec::new();
                    for i in 0..self.height {
                        for j in 0..self.width {
                            if !self.cells[i][j].visited {
                                remaining_cells.push((j as i32, i as i32));
                            }
                        }
                    }
                    erase_path = false;
                }
                current_path_steps = Vec::new();
                if remaining_cells.is_empty() {
                    println!("No more remaining cells!");
                    break;
                }
                (cell_x, cell_y) = *remaining_cells.choose(&mut rng).unwrap();
                current_path_steps.push((cell_x, cell_y));
                self.cells[cell_y as usize][cell_x as usize].visited = true;
                remaining_cells = remaining_cells.into_iter().filter(|(xi, yj)| !self.cells[*yj as usize][*xi as usize].visited).collect::<Vec<(i32, i32)>>();
                //println!("{:?} \nLeft: {}", remaining_cells, remaining_cells.len());
                //println!("New cell chosen: {},{}", cell_x, cell_y);
                prev_move_x = 0;
                prev_move_y = 0;

            }

            //self.print();
            //println!("Remaining cells left: {}", remaining_cells.len());
            //println!("Current steps: {}", current_path_steps.len());
            //sleep(Duration::from_millis(50));
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn set(&mut self, x: usize, y: usize, c: char) {
        self.grid[y][x] = c
    }

    fn print(&self) {
        println!("");
        println!("{}{}", "# ", '#'.to_string().repeat(self.width));
        for y in 0..self.height {
            print!("#");
            for x in 0..self.width {
                if self.paths[y][x].visited {
                    print!("{}", self.grid[y][x].to_string().red());
                } else {
                    print!("{}", self.grid[y][x].to_string().normal());
                }
            }
            println!("#");
        }
        println!("{}{}", '#'.to_string().repeat(self.width), " #");
    }
}
