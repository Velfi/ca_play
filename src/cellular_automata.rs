use crate::utils::{get_left_neighbour_index_wrapping, get_right_neighbour_index_wrapping};
use bit_set::BitSet;
use image::{ImageBuffer, Rgb, RgbImage};
use std::fmt::{Display, Error, Formatter};

pub enum RowStartPosition {
    Left,
    Right,
    Center,
}

pub enum BorderHandling {
    Alive,
    Dead,
    Wrapping,
}

pub struct Generation {
    cells: Vec<bool>,
}

impl Display for Generation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let generation_as_string: String = self
            .cells
            .iter()
            .map(|cell_state| if *cell_state { '■' } else { '□' })
            .collect();
        write!(f, "{}", generation_as_string)
    }
}

impl Generation {
    pub fn new(width: usize, start_position: RowStartPosition) -> Self {
        assert!(
            width > 0,
            "You can't create 0 width rows, please create something of width 1 or larger."
        );

        let start_point = match start_position {
            RowStartPosition::Left => 0,
            RowStartPosition::Right => width - 1,
            RowStartPosition::Center => width / 2,
        };

        Generation {
            cells: (0..width)
                .map(|index: usize| index == start_point)
                .collect(),
        }
    }

    pub fn get_cell_state(&self, cell_index: usize) -> Option<bool> {
        if cell_index + 1 >= self.cells.len() {
            return None;
        }

        Some(self.cells[cell_index])
    }

    pub fn get_next_generation(&self, rules: u8, border_handling: &BorderHandling) -> Self {
        Generation {
            cells: self
                .cells
                .iter()
                .enumerate()
                .map(|(index, _)| self.get_next_cell_state(index, rules, border_handling))
                .collect(),
        }
    }

    fn get_next_cell_state(
        &self,
        cell_index: usize,
        rules: u8,
        border_handling: &BorderHandling,
    ) -> bool {
        use BorderHandling::*;

        let (left_neighbour, current_state, right_neighbour) = match border_handling {
            Wrapping => self.get_cell_and_neighbours_wrapping(cell_index),
            Alive => self.get_cell_and_neighbours(cell_index, true),
            Dead => self.get_cell_and_neighbours(cell_index, false),
        };

        get_next_cell_state(left_neighbour, current_state, right_neighbour, rules)
    }

    fn get_cell_and_neighbours_wrapping(&self, cell_index: usize) -> (bool, bool, bool) {
        let left_neighbour_index: usize =
            get_left_neighbour_index_wrapping(cell_index, self.cells.len());
        let right_neighbour_index: usize =
            get_right_neighbour_index_wrapping(cell_index, self.cells.len());

        (
            self.cells[left_neighbour_index],
            self.cells[cell_index],
            self.cells[right_neighbour_index],
        )
    }

    fn get_cell_and_neighbours(
        &self,
        cell_index: usize,
        neighbour_default: bool,
    ) -> (bool, bool, bool) {
        let left_neighbour_index: isize = cell_index as isize - 1;
        let left_neighbour;

        if left_neighbour_index < 0 {
            left_neighbour = neighbour_default;
        } else {
            left_neighbour = self.cells[left_neighbour_index as usize];
        }

        let right_neighbour_index: usize = cell_index + 1;
        let right_neighbour;

        if right_neighbour_index >= self.cells.len() {
            right_neighbour = neighbour_default;
        } else {
            right_neighbour = self.cells[right_neighbour_index as usize];
        }

        (left_neighbour, self.cells[cell_index], right_neighbour)
    }
}

pub struct ElementaryCellularAutomata {
    generations: Vec<Generation>,
    width: usize,
    rule: u8,
    border_handling: BorderHandling,
}

impl Display for ElementaryCellularAutomata {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let generations: Vec<String> = self
            .generations
            .iter()
            .map(|generation| generation.to_string())
            .collect();

        let generations_as_string = generations.join("\n");
        write!(f, "{}", generations_as_string)
    }
}

impl Default for ElementaryCellularAutomata {
    fn default() -> Self {
        ElementaryCellularAutomata {
            generations: vec![Generation::new(100, RowStartPosition::Center)],
            width: 100,
            rule: 110,
            border_handling: BorderHandling::Dead,
        }
    }
}

impl ElementaryCellularAutomata {
    pub fn new(
        rule: u8,
        width: usize,
        start_position: RowStartPosition,
        border_handling: BorderHandling,
    ) -> Self {
        assert!(width > 0, "Width must be greater than zero");

        ElementaryCellularAutomata {
            generations: vec![Generation::new(width, start_position)],
            width,
            rule,
            border_handling,
        }
    }

    pub fn update(&mut self) {
        let next_generation = self
            .get_next_generation()
            .expect("Failed to generate the next generation.");

        self.generations.push(next_generation);
    }

    pub fn as_image_buffer(&self) -> RgbImage {
        ImageBuffer::from_fn(
            self.width as u32,
            self.generations.len() as u32,
            |x, y| match self.generations[y as usize].get_cell_state(x as usize) {
                Some(false) => Rgb([255u8, 255u8, 255u8]),
                _ => Rgb([0u8, 0u8, 0u8]),
            },
        )
    }

    fn get_next_generation(&self) -> Option<Generation> {
        if let Some(current_generation) = self.generations.last() {
            return Some(current_generation.get_next_generation(self.rule, &self.border_handling));
        }

        None
    }
}

fn get_next_cell_state(
    left_neighbour: bool,
    current_state: bool,
    right_neighbour: bool,
    rules: u8,
) -> bool {
    let rules_bitset = BitSet::from_bytes(&[rules]);

    match (left_neighbour, current_state, right_neighbour) {
        (true, true, true) => rules_bitset.contains(0),
        (true, true, false) => rules_bitset.contains(1),
        (true, false, true) => rules_bitset.contains(2),
        (true, false, false) => rules_bitset.contains(3),
        (false, true, true) => rules_bitset.contains(4),
        (false, true, false) => rules_bitset.contains(5),
        (false, false, true) => rules_bitset.contains(6),
        (false, false, false) => rules_bitset.contains(7),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_next_cell_state_rule_30(
        left_neighbour: bool,
        current_state: bool,
        right_neighbour: bool,
    ) -> bool {
        match (left_neighbour, current_state, right_neighbour) {
            (true, true, true) => false,
            (true, true, false) => false,
            (true, false, true) => false,
            (true, false, false) => true,
            (false, true, true) => true,
            (false, true, false) => true,
            (false, false, true) => true,
            (false, false, false) => false,
        }
    }

    #[test]
    fn test_get_next_cell_state() {
        assert_eq!(
            get_next_cell_state(true, true, true, 30),
            get_next_cell_state_rule_30(true, true, true)
        );
        assert_eq!(
            get_next_cell_state(true, true, false, 30),
            get_next_cell_state_rule_30(true, true, false)
        );
        assert_eq!(
            get_next_cell_state(true, false, true, 30),
            get_next_cell_state_rule_30(true, false, true)
        );
        assert_eq!(
            get_next_cell_state(true, false, false, 30),
            get_next_cell_state_rule_30(true, false, false)
        );
        assert_eq!(
            get_next_cell_state(false, true, true, 30),
            get_next_cell_state_rule_30(false, true, true)
        );
        assert_eq!(
            get_next_cell_state(true, true, false, 30),
            get_next_cell_state_rule_30(true, true, false)
        );
        assert_eq!(
            get_next_cell_state(false, false, true, 30),
            get_next_cell_state_rule_30(false, false, true)
        );
        assert_eq!(
            get_next_cell_state(false, false, false, 30),
            get_next_cell_state_rule_30(false, false, false)
        );
    }
}
