use std::fmt;
use std::io;
use std::iter;

use solver;

const WIDTH : usize = 9;
const SUBWIDTH : usize = 3;

#[derive(Copy, Clone, Debug)]
pub struct State {
    cells: [[CellValue; WIDTH]; WIDTH],

    // Ideas for speed improvements:
    //  - is_win(): keep track of the number of unknown cells.
    //  - actions(): maintain for each row/column/block the list of possible values.
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellValue {
    NotSet,
    Digit(usize),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cell {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Action {
    cell: Cell,
    value: CellValue,
}

impl solver::GameState for State {
    type Action = Action;

    fn is_win(&self) -> bool {
        for row in self.cells.iter() {
            for val in row.iter() {
                if *val == CellValue::NotSet {
                    return false;
                }
            }
        }
        return true;
    }

    fn actions(&self) -> Vec<Action> {
        struct CellActions {
            cell: Cell,
            possible_values: Vec<CellValue>,
        }

        let mut cells = Vec::<CellActions>::new();

        for (x, row) in self.cells.iter().enumerate() {
            for (y, current_value) in row.iter().enumerate() {
                if let CellValue::Digit(_) = *current_value {
                    continue;
                }

                let mut possible_values = [true; WIDTH];
                for val in row {
                    if let CellValue::Digit(d) = *val {
                        possible_values[d-1] = false;
                    }
                }
                for xx in 0..WIDTH {
                    if let CellValue::Digit(d) = self.cells[xx][y] {
                        possible_values[d-1] = false;
                    }
                }
                for xx in SUBWIDTH*(x/SUBWIDTH)..SUBWIDTH*(x/SUBWIDTH + 1) {
                    for yy in SUBWIDTH*(y/SUBWIDTH)..SUBWIDTH*(y/SUBWIDTH + 1) {
                        if let CellValue::Digit(d) = self.cells[xx][yy] {
                            possible_values[d-1] = false;
                        }
                    }
                }

                let mut cell = CellActions {
                    cell: Cell{x:x, y:y},
                    possible_values: vec!(),
                };


                for (v, is_possible) in possible_values.iter().enumerate() {
                    if *is_possible {
                        cell.possible_values.push(CellValue::Digit(v+1));
                    }
                }
                if cell.possible_values.is_empty() {
                    // This cell is impossible to fill, the game is blocked here.
                    return vec![];
                }
                cells.push(cell);
            }
        }

        cells.sort_by_key(|cell| cell.possible_values.len());

        iter::repeat(cells[0].cell).zip(cells[0].possible_values.iter())
            .map(|(cell, val)| Action{ cell:cell, value: *val })
            .collect()
    }

    fn apply(&mut self, action: &Action) {
        self.cells[action.cell.x][action.cell.y] = action.value;
    }

    fn revert(&mut self, action: &Action) {
        self.cells[action.cell.x][action.cell.y] = CellValue::NotSet;
    }
}

#[derive(Debug)]
pub enum ReadError {
    Io(io::Error),
    InvalidLine { line_index: usize, line: String },
}

impl From<io::Error> for ReadError {
    fn from(err: io::Error) -> ReadError {
        ReadError::Io(err)
    }
}


impl State {
    pub fn read<R: io::BufRead>(mut reader: R) -> Result<State, ReadError> {
        let mut state = State::new();

        let mut line = String::new();

        for (i, row) in state.cells.iter_mut().enumerate() {
            line.clear();
            try!(reader.read_line(&mut line));

            if !State::parse_line(line.trim(), &mut *row) {
                return Err(ReadError::InvalidLine {
                    line_index: i,
                    line: line,
                });
            }
        }

        Ok(state)
    }

    fn new() -> State {
        State {
            cells: [[CellValue::NotSet; WIDTH]; WIDTH],
        }
    }

    fn parse_line(line: &str, row: &mut [CellValue; WIDTH]) -> bool {
        if line.len() != row.len() {
            return false;
        }
        for (val, c) in row.iter_mut().zip(line.chars()) {
            if c == '0' || c == '.' || c == '#' {
                *val = CellValue::NotSet;
            } else if let Ok(i) = c.to_string().parse() {
                *val = CellValue::Digit(i);
            } else {
                return false;
            }
        }
        true
    }

}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for row in self.cells.iter() {
            for val in row.iter() {
                match *val {
                    CellValue::NotSet => try!(write!(f, "{}", '.')),
                    CellValue::Digit(d @ 1...WIDTH) => try!(write!(f, "{}", d)),
                    _ => return Err(fmt::Error),
                }
            }
            try!(write!(f, "\n"));
        }
        Ok(())
    }
}
