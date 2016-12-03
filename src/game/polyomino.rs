use std::collections::HashSet;
use std::vec::Vec;
use renderer::{Point, Renderable, Renderer};
use std::cmp::min;
use std::fmt;


pub type Cell = (i8, i8);
type Coord = (i8, i8);


pub struct Polyomino {
    cells: HashSet<Cell>
}

impl Polyomino {
    pub fn new(cells: HashSet<Cell>) -> Polyomino {
        // Translate the cells to origin
        let translated_cells = Polyomino::translate(cells);

        Polyomino {
            cells: translated_cells
        }
    }

    pub fn rotate(&mut self, clockwise: bool) {
        self.cells = self.get_rotated(clockwise);
    }

    pub fn get_rotated(&self, clockwise: bool) -> HashSet<Cell> {
        // Rotate each cell
        let mut rotated_cells = HashSet::new();

        for cell in self.cells.iter() {
            rotated_cells.insert((
                (-1 + 2 * (!clockwise as i8)) * cell.1,
                (-1 + 2 * (clockwise as i8)) * cell.0
            ));
        }

        Polyomino::translate(rotated_cells)
    }

    fn translate(cells: HashSet<Cell>) -> HashSet<Cell> {
        // Find the smallest x and y for each cell
        let num_cells = cells.len() as i8;

        // Can't be bigger than num_cells
        let mut min_x = num_cells;
        let mut min_y = num_cells;

        for cell in cells.iter() {
            if cell.0 < min_x {
                min_x = min(cell.0, min_x);
            }

            if cell.1 < min_y {
                min_y = min(cell.1, min_y);
            }
        }

        return cells.iter().map(
            |&cell| (cell.0 - min_x, cell.1 - min_y)
        ).collect();
    }

    pub fn add_cell(&self, cell: Cell) -> Polyomino {
        let mut cells = self.cells.clone();
        cells.insert(cell);

        Polyomino::new(cells)
    }

    pub fn potential_cells(&self) -> HashSet<Cell> {
        let mut potential: HashSet<Cell> = HashSet::new();

        for cell in self.cells.iter() {
            potential.insert((cell.0 - 1, cell.1));
            potential.insert((cell.0 + 1, cell.1));
            potential.insert((cell.0, cell.1 - 1));
            potential.insert((cell.0, cell.1 + 1));
        }

        potential.difference(&self.cells).map(|x| x.clone()).collect()
    }
}

impl PartialEq for Polyomino {
    fn eq(&self, other: &Polyomino) -> bool {
        // Make sure none of the rotations match
        for i in 0..4 {
            let rotated_cells = other.get_rotated(true);  // Rotate clockwise

            if self.cells == rotated_cells {
                return true;
            }
        }

        false
    }
}

impl Renderable for Polyomino {
    fn render(&self, pos: Point, renderer: &mut Renderer) {
        for cell in self.cells.iter() {
            renderer.block((
                (cell.0 as i32 + pos.0) * 2,
                cell.1 as i32 + pos.1
            ));
        }
    }
}

impl fmt::Display for Polyomino {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.cells)
    }
}

pub fn generate(complexity: u8) -> Vec<Polyomino> {
    // Generate a list of fixed polyominoes up to complexity

    let mut polyominoes = Vec::new();

    if complexity == 1 {
        let mut initial_polyomino = HashSet::new();
        initial_polyomino.insert((0, 0));

        polyominoes.push(Polyomino::new(initial_polyomino));
    } else if complexity > 1 {
        for polyomino in generate(complexity - 1).iter() {
            for cell in polyomino.potential_cells().iter() {
                let new_polyomino = polyomino.add_cell(*cell);
                if !(polyominoes.contains(&new_polyomino)) {
                    polyominoes.push(new_polyomino)
                }
            }
        }
    }

    polyominoes
}