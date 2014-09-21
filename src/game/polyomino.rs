use std::collections::HashSet;
use std::vec::Vec;
use renderer::Renderable;
use renderer::Renderer;


type Cell = (uint, uint);
type Coord = (int, int);


pub struct Polyomino {
    cells: HashSet<Cell>
}

impl Polyomino {
    pub fn new(cells: HashSet<Cell>) -> Polyomino {
        // Find the smallest x and y for each cell
        let num_cells = cells.len();
        let mut min_x = num_cells;
        let mut min_y = num_cells;

        // Translate the cells to origin
        if cells.len() > 0 {
            // Can't be bigger than num_cells

            for cell in cells.iter() {
                if cell.val0() < min_x {
                    min_x = cell.val0();
                }

                if cell.val1() < min_y {
                    min_y = cell.val1();
                }
            }
        }

        Polyomino {
            cells: cells.iter().map(
                |&cell| (cell.val0() - min_x, cell.val1() - min_y)
            ).collect()
        }
    }

    pub fn add_cell(&self, cell: Cell) -> Polyomino {
        let mut cells = self.cells.clone();
        cells.insert(cell);

        Polyomino { cells: cells }
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
        return self.cells == other.cells;
    }
}

impl Renderable for Polyomino {
    fn render(&self, pos: Coord, renderer: &Renderer) {
        for cell in self.cells.iter() {
            renderer.block(
                (
                    cell.val0() as int + pos.val0(),
                    cell.val1() as int + pos.val1(),
                ),
                2  // Color
            );
        }
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