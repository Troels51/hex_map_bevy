use std::{
    collections::HashMap,
    ops::{BitAndAssign, BitOrAssign},
};

use bevy::{
    prelude::{Component, Resource},
    reflect::{TypePath, TypeUuid},
};
use hex2d::{Angle, Coordinate};
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Side {
    Ocean,
    Grass,
    Forest,
    Hills,
    Road,
    Desert,
    Mountain,
    Any,
}

// 2 sides are equal if they are equal, or if one of them is Any
impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (_, Side::Any) => true,
            (Side::Any, _) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

pub const HEX_SIDES: usize = 6;

type HexSides = [Side; HEX_SIDES];

// Check if 2 side descriptions match
fn match_sides(side1: &HexSides, side2: &HexSides) -> bool {
    // We concat side1 with itself, and check if side2 is contained in it
    let v: Vec<Side> = side1.iter().chain(side1.iter()).cloned().collect();
    for seq in v.windows(6) {
        if seq.eq(side2) {
            return true;
        }
    }
    false
}

#[derive(Serialize, Deserialize, TypeUuid, Debug, Clone, PartialEq, Component, TypePath)]
#[uuid = "64271346-f11a-4736-ae17-6876acf29761"]
pub struct Hex {
    pub sides: HexSides,
    pub name: String,
    pub rotation: u8,
}

impl Default for Hex {
    fn default() -> Self {
        Self {
            sides: [Side::Any; HEX_SIDES],
            name: String::from(""),
            rotation: 0,
        }
    }
}

impl Hex {
    pub fn get_matching_rotations(&self, matching: &HexSides) -> Vec<u8> {
        //There is probably a better way to do this :(
        let mut rotations: Vec<u8> = Vec::new();
        let mut sides = self.sides.clone();
        for i in 0..6 {
            if sides == *matching {
                rotations.push(i as u8);
            }
            sides.rotate_right(1);
        }
        rotations
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotations(u8);

impl Rotations {
    fn set(&mut self, index: u8) {
        self.0.bitor_assign(1 << index)
    }
    fn clear(&mut self, index: u8) {
        self.0.bitand_assign(1 << index)
    }
    pub fn is_valid(&self, index: u8) -> bool {
        (self.0 >> index & 1) != 0
    }
    fn entropy(&self) -> u32 {
        self.0.count_ones() // Why does count ones return u32 for u8? Optimizations?
    }
    fn zero(&self) -> bool {
        self.0 == 0
    }
    fn clear_all(&mut self) {
        self.0 = 0
    }
}

impl Default for Rotations {
    fn default() -> Self {
        Self(0b00111111)
    }
}

pub struct RotationsIterator {
    rotations: Rotations,
    index: u8,
}
impl Iterator for RotationsIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 6 {
            let next = Some(self.rotations.is_valid(self.index));
            self.index += 1;
            next
        } else {
            None
        }
    }
}

impl IntoIterator for Rotations {
    type Item = bool;
    type IntoIter = RotationsIterator;
    fn into_iter(self) -> Self::IntoIter {
        RotationsIterator {
            rotations: self,
            index: 0,
        }
    }
}

impl PossiblitySpace {
    fn entropy(&self) -> u32 {
        self.valid_rotations
            .iter()
            .fold(0, |acc, rotations| acc + rotations.entropy())
    }
    fn new(size: usize) -> Self {
        Self {
            valid_rotations: vec![Rotations::default(); size],
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PossiblitySpace {
    // For each hex, a list of valid rotations
    pub valid_rotations: Vec<Rotations>,
}

#[derive(Debug)]
struct Cell {
    hex: Hex,
    possibility_space: PossiblitySpace,
}

impl Cell {
    fn new(len: usize) -> Self {
        Cell {
            hex: Hex::default(),
            possibility_space: PossiblitySpace::new(len),
        }
    }
}
#[derive(Debug, Resource)]
pub struct Board {
    hexes: HashMap<Coordinate, Cell>,
    possible_hexes: Vec<Hex>,
}

impl Board {
    pub fn new(possible_hexes: Vec<Hex>) -> Board {
        Board {
            hexes: HashMap::from([(Coordinate::new(0, 0), Cell::new(possible_hexes.len()))]),
            possible_hexes: possible_hexes,
        }
    }

    pub fn clear_board(&mut self) {
        self.hexes = HashMap::from([(Coordinate::new(0, 0), Cell::new(self.possible_hexes.len()))]);
    }

    pub fn get(&self, coordinate: hex2d::Coordinate) -> Option<&Hex> {
        Some(&self.hexes.get(&coordinate)?.hex)
    }

    pub fn set(&mut self, coordinate: hex2d::Coordinate, hex: Hex) {
        let current_hex = hex.clone();
        let tile: &mut Cell = self.hexes.get_mut(&coordinate).unwrap();
        tile.hex = hex;
        tile.possibility_space.valid_rotations.clear();
        // After we set the hex, we propagate the constraints to the sorounding hexes
        for i in coordinate.neighbors() {
            if let Some(direction) = i.direction_to_cw(coordinate) {
                // We find the direction of the neighbour and then use that to set the side socket to the corresponding original side
                let tile = self
                    .hexes
                    .entry(i)
                    .or_insert_with(|| Cell::new(self.possible_hexes.len()));
                tile.hex.sides[direction.to_int::<i8>() as usize] =
                    current_hex.sides[(-direction + Angle::from_int(-(current_hex.rotation as i8)))
                        .to_int::<i8>() as usize];
            }
            // collapse wave function at that coordinate
            self.collapse_wave_function(i);
        }
    }

    fn collapse_wave_function(&mut self, coordinate: hex2d::Coordinate) {
        let hex = self.get(coordinate).unwrap().clone();
        let possibility_space: &mut PossiblitySpace =
            &mut self.hexes.get_mut(&coordinate).unwrap().possibility_space;
        for (index, rotations) in possibility_space.valid_rotations.iter_mut().enumerate() {
            if rotations.zero() {
                continue;
            }
            let possible = self.possible_hexes.get(index).unwrap();
            let matches = possible.get_matching_rotations(&hex.sides); // Why is get_matching_rotations not symmetric?
            rotations.clear_all();
            for i in &matches {
                rotations.set(*i);
            }
        }
    }

    pub fn get_possible_hexes_for_coordinate(
        &self,
        coordinate: hex2d::Coordinate,
    ) -> Vec<(Hex, Rotations)> {
        let space = &self.hexes.get(&coordinate).unwrap().possibility_space;
        space
            .valid_rotations
            .iter()
            .enumerate()
            .filter_map(|(index, rotations)| {
                if rotations.zero() {
                    None
                } else {
                    Some((
                        self.possible_hexes.get(index).unwrap().clone(),
                        rotations.clone(),
                    ))
                }
            })
            .collect()
    }

    pub fn get_minimal_entropy_coordinate(&self) -> Option<Coordinate> {
        let mut min_entropy = u32::MAX;
        let mut min_coordinate: Option<Coordinate> = Option::None;
        for (coordinate, cell) in &self.hexes {
            let entropy = cell.possibility_space.entropy();
            if entropy < min_entropy && entropy != 0 {
                min_coordinate = Some(*coordinate);
                min_entropy = entropy;
            }
        }
        min_coordinate
    }

    /// Get possible hexes that match the side description
    /// they are not prerotated, so if a hex matches, you need to rotate it in place afterwards
    pub fn get_possible_matching_hexes(&self, sides: &HexSides) -> Vec<&Hex> {
        let filtered: Vec<&Hex> = self
            .possible_hexes
            .iter()
            .filter(|p_hex| match_sides(&p_hex.sides, sides))
            .collect();
        filtered
    }
}

#[test]
fn set_hex_test() {
    use hex2d::{Coordinate, Direction::*};
    let mut b = Board::new(vec![]);
    let hex = Hex {
        sides: [
            Side::Grass,
            Side::Ocean,
            Side::Ocean,
            Side::Ocean,
            Side::Ocean,
            Side::Ocean,
        ],
        name: String::from("String"),
        rotation: 0,
    };
    let center = Coordinate::new(1, 1);
    b.set(center, hex);
    //We check each of the neighbours to see if the correct place has been set, with memcmp because ANY matches everything
    assert_eq!(
        core::mem::discriminant(&b.get(center + YZ).unwrap().sides[3]),
        core::mem::discriminant(&Side::Grass)
    );
    assert_eq!(
        core::mem::discriminant(&b.get(center + XZ).unwrap().sides[4]),
        core::mem::discriminant(&Side::Ocean)
    );
    assert_eq!(
        core::mem::discriminant(&b.get(center + XY).unwrap().sides[5]),
        core::mem::discriminant(&Side::Ocean)
    );
    assert_eq!(
        core::mem::discriminant(&b.get(center + ZY).unwrap().sides[0]),
        core::mem::discriminant(&Side::Ocean)
    );
    assert_eq!(
        core::mem::discriminant(&b.get(center + ZX).unwrap().sides[1]),
        core::mem::discriminant(&Side::Ocean)
    );
    assert_eq!(
        core::mem::discriminant(&b.get(center + YX).unwrap().sides[2]),
        core::mem::discriminant(&Side::Ocean)
    );
}

#[test]
fn set_rotated_hex_test() {
    use hex2d::{Coordinate, Direction::*};
    let mut b = Board::new(vec![]);
    let hex = Hex {
        sides: [
            Side::Grass,
            Side::Ocean,
            Side::Ocean,
            Side::Ocean,
            Side::Ocean,
            Side::Ocean,
        ],
        name: String::from("String"),
        rotation: 1, //Rotate 1
    };
    let center = Coordinate::new(1, 1);
    b.set(center, hex);
    //We check each of the neighbours to see if the correct place has been set, done with memcmp, because ANY matches everything
    assert_eq!(
        core::mem::discriminant(&b.get(center + XZ).unwrap().sides[4]),
        core::mem::discriminant(&Side::Grass)
    );
    assert_eq!(
        core::mem::discriminant(&b.get(center + XY).unwrap().sides[5]),
        core::mem::discriminant(&Side::Ocean)
    );
    assert_eq!(
        core::mem::discriminant(&b.get(center + ZY).unwrap().sides[0]),
        core::mem::discriminant(&Side::Ocean)
    );
    assert_eq!(
        core::mem::discriminant(&b.get(center + ZX).unwrap().sides[1]),
        core::mem::discriminant(&Side::Ocean)
    );
    assert_eq!(
        core::mem::discriminant(&b.get(center + YX).unwrap().sides[2]),
        core::mem::discriminant(&Side::Ocean)
    );
    assert_eq!(
        core::mem::discriminant(&b.get(center + YZ).unwrap().sides[3]),
        core::mem::discriminant(&Side::Ocean)
    );
}

#[test]
fn match_hex_side_test() {
    //These 2 should match by rotating 1
    let a = [
        Side::Grass,
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
    ];
    let b = [
        Side::Ocean,
        Side::Grass,
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
    ];
    assert!(match_sides(&a, &b));

    //These 2 should not match
    let a = [
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
    ];
    let b = [
        Side::Ocean,
        Side::Grass,
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
    ];
    assert!(!match_sides(&a, &b));

    //Any should match everything
    let a = [
        Side::Any,
        Side::Any,
        Side::Any,
        Side::Any,
        Side::Any,
        Side::Any,
    ];
    let b = [
        Side::Ocean,
        Side::Grass,
        Side::Hills,
        Side::Road,
        Side::Desert,
        Side::Mountain,
    ];
    assert!(match_sides(&a, &b));
}

#[test]
fn possibility_test() {
    use hex2d::{Coordinate, Direction::*};

    let h1 = Hex {
        name: String::from("A001"),
        sides: [
            Side::Grass,
            Side::Grass,
            Side::Grass,
            Side::Ocean,
            Side::Ocean,
            Side::Ocean,
        ],
        rotation: 0,
    };
    let mut b = Board::new(vec![h1.clone()]);

    let center = Coordinate::new(1, 1);
    b.set(center, h1);
    assert_eq!(b.get(center + YZ).unwrap().sides[3], Side::Grass);
    assert_eq!(
        core::mem::discriminant(&b.get(center + YZ).unwrap().sides[3]),
        core::mem::discriminant(&Side::Grass)
    ); //Any matches everything, so we need to test the actual memory
       //The top side is GRASS, so a match is possible if the hex is rotated, 1,2 or 3 times
    let possible = b.get_possible_hexes_for_coordinate(center + YZ);

    let rotations: Rotations = possible.first().unwrap().1.clone();
    assert_eq!(rotations, Rotations(0b1110));
}

#[test]
fn possible_matching_test() {
    use hex2d::{Coordinate, Direction::*};
    let h1 = Hex {
        name: String::from("A001"),
        sides: [
            Side::Grass,
            Side::Grass,
            Side::Grass,
            Side::Ocean,
            Side::Ocean,
            Side::Ocean,
        ],
        rotation: 0,
    };
    let mut b = Board::new(vec![h1.clone()]);

    let center = Coordinate::new(1, 1);
    b.set(center, h1.clone());
    assert_eq!(b.get(center + YZ).unwrap().sides[3], Side::Grass);
    assert_eq!(
        core::mem::discriminant(&b.get(center + YZ).unwrap().sides[3]),
        core::mem::discriminant(&Side::Grass)
    ); //Any matches everything, so we need to test the actual memory
       //The top side is GRASS, so a match is possible if the hex is rotated, 1,2 or 3 times
    let possible = b.get_possible_matching_hexes(&[
        Side::Grass,
        Side::Grass,
        Side::Grass,
        Side::Ocean,
        Side::Ocean,
        Side::Ocean,
    ]);
    assert_eq!(possible[0], &h1);
}

#[test]
fn matching_rotations_test() {
    let h1 = Hex {
        name: String::from("A001"),
        sides: [
            Side::Grass,
            Side::Grass,
            Side::Grass,
            Side::Ocean,
            Side::Ocean,
            Side::Ocean,
        ],
        rotation: 0,
    };
    let rotations = h1.get_matching_rotations(&[
        Side::Any,
        Side::Any,
        Side::Any,
        Side::Grass,
        Side::Any,
        Side::Any,
    ]);
    assert_eq!(rotations, vec![1, 2, 3]);
}
