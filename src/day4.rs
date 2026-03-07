#![allow(unused)]

use std::{fs::read_to_string, path::PathBuf};

pub fn solve_p2() {
    let input = read_to_string(PathBuf::from("day4.txt")).unwrap();
    let mut cells = Cells::new(
        input
            .trim()
            .bytes()
            .map(Cell::from_ch)
            .collect::<Vec<Cell>>(),
    );
    // let solution = input.count_accessible();
    let mut solution = 0;
    let mut removed = cells.remove_accessible_cells();
    while removed > 0 {
        solution += removed;
        removed = cells.remove_accessible_cells();
    }

    println!("{}", solution);
}

pub struct Cells {
    cells: Vec<Cell>,
    stride: usize,
    length: usize,
}

impl Cells {
    fn get_adjacent_cells_index(&self, i: usize) -> impl Iterator<Item = usize> {
        [
            i.checked_sub(self.stride - 1),
            i.checked_sub(self.stride),
            i.checked_sub(self.stride + 1),
            i.checked_sub(1),
            Some(i + 1),
            Some(i + self.stride - 1),
            Some(i + self.stride),
            Some(i + self.stride + 1),
        ]
        .into_iter()
        .filter_map(|x| match x {
            Some(a) if a < self.length => Some(a),
            _ => None,
        })
    }
    fn is_accessible(&self, index: usize) -> bool {
        self.get_adjacent_cells_index(index)
            .filter(|x| matches!(self.cells[*x as usize], Cell::PaperRoll))
            .count()
            < 4
    }

    pub fn _count_accessible(&self) -> usize {
        (0..self.length)
            .filter(|x| matches!(self.cells[*x], Cell::PaperRoll) && self.is_accessible(*x))
            .count()
    }

    pub(crate) fn new(cells: Vec<Cell>) -> Self {
        Self {
            length: cells.len(),
            stride: cells
                .iter()
                .take_while(|x| !matches!(x, Cell::NewL))
                .count()
                + 1,
            cells,
        }
    }

    fn remove_paper_roll(&mut self, index: usize) {
        assert!(matches!(self.cells[index], Cell::PaperRoll));
        self.cells[index] = Cell::Removed;
    }

    pub fn remove_accessible_cells(&mut self) -> u64 {
        let mut removed = 0;
        for i in 0..self.length {
            if (matches!(self.cells[i], Cell::PaperRoll) && self.is_accessible(i)) {
                self.remove_paper_roll(i);
                removed += 1;
            }
        }
        removed
    }
}

pub enum Cell {
    NewL,
    PaperRoll,
    Dot,
    Invalid,
    Removed,
}

impl Cell {
    pub fn from_ch(ch: u8) -> Cell {
        match ch {
            b'\n' => Cell::NewL,
            b'.' => Cell::Dot,
            b'@' => Cell::PaperRoll,
            _ => Cell::Invalid,
        }
    }
}
