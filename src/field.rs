use std::collections::HashMap;
use std::fmt;

use crate::monster::Monster;
use crate::terrain::Terrain;

pub struct Field {
    // Dimensions of field
    pub length: i32,
    pub width: i32,
    // 2D vector representation what terrain each grid of the field is
    pub area: Vec<Vec<Terrain>>,
    // Monsters encounterable on this field.
    // Monster mapped to chance of encounter (ex. Slime, 0.6)
    pub monsters: HashMap<Monster, f32>
}

impl Field {
    pub fn new(length: i32, width: i32) -> Self {
        Field {
            length,
            width,
            area: Self::fill_area(length, width),
            monsters: HashMap::new()
        }
    }

    fn fill_area(length: i32, width: i32) -> Vec<Vec<Terrain>> {
        let mut result: Vec<Vec<Terrain>> = Vec::with_capacity(length as usize);
        for i in 0..length {
            result.push(Vec::with_capacity(width as usize));
            for _j in 0..width {
                result[i as usize].push(Terrain::Land);
            }
        }
        return result;
    }
}

impl fmt::Display for Field {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut result: String = "".to_string();

        for i in 0..self.area.len() {
            for j in 0..self.area[0].len() {
                if j < self.area[0].len() - 1 {
                    result += format!("{} ", self.area[i as usize][j as usize]).as_str();
                } else {
                    result += format!("{}\n", self.area[i as usize][j as usize]).as_str();
                }
            }
        }

        return write!(fmt, "{}", result);
    }
}
