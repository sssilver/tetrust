use std::collections::HashSet;
use std::vec::Vec;


type Cell = (int, int);


pub struct Polyomino {
    cells: HashSet<Cell>
}

impl Polyomino {
    pub fn new(cells: HashSet<Cell>) -> Polyomino {
        Polyomino {cells: cells}
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