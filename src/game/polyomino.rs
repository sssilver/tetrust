use std::collections::HashSet;
use std::vec::Vec;
use renderer::Renderable;
use renderer::Renderer;
use std::cmp::min;
use std::fmt;


type Cell = (int, int);
type Coord = (int, int);


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
                (-1 + 2 * (!clockwise as int)) * cell.val1(),
                (-1 + 2 * (clockwise as int)) * cell.val0()
            ));
        }

        Polyomino::translate(rotated_cells)
    }

    fn translate(cells: HashSet<Cell>) -> HashSet<Cell> {
        // Find the smallest x and y for each cell
        let num_cells = cells.len();

        // Can't be bigger than num_cells
        let mut min_x = num_cells as int;
        let mut min_y = num_cells as int;

        for cell in cells.iter() {
            if cell.val0() < min_x {
                min_x = min(cell.val0(), min_x);
            }

            if cell.val1() < min_y {
                min_y = min(cell.val1(), min_y);
            }
        }

        return cells.iter().map(
            |&cell| (cell.val0() - min_x, cell.val1() - min_y)
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
            potential.insert((cell.val0() - 1, cell.val1()));
            potential.insert((cell.val0() + 1, cell.val1()));
            potential.insert((cell.val0(), cell.val1() - 1));
            potential.insert((cell.val0(), cell.val1() + 1));
        }

        potential.difference(&self.cells).map(|x| x.clone()).collect()
    }
}

impl PartialEq for Polyomino {
    fn eq(&self, other: &Polyomino) -> bool {
        // Make sure none of the rotations match
        for i in range(0u8, 4) {
            let rotated_cells = other.get_rotated(true);  // Rotate clockwise

            if self.cells == rotated_cells {
                return true;
            }
        }

        false
    }
}

impl Renderable for Polyomino {
    fn render(&self, pos: Coord, renderer: &Renderer) {
        for cell in self.cells.iter() {
            renderer.block(
                (
                    ((cell.val0() as int) + pos.val0()) * 2,
                    (cell.val1() as int) + pos.val1()
                ),
                2  // Color
            );
        }
    }
}

impl fmt::Show for Polyomino {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cells)
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