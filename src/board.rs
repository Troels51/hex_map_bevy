use bevy::{prelude::{Component, Resource, Handle}, reflect::{TypeUuid, TypePath}};
use hex2d::{Angle, Position};
use serde::{Deserialize, Serialize};

use crate::loading::hex_descriptions::HexDescriptions;



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

/// If we save the possiblity space for each coordinate
/// It would be nr_hexes * 6 * boardsize = 100 * 6 * 100 = 60000
///
pub const HEX_SIDES: usize = 6;

type HexSides = [Side; HEX_SIDES];

// Check if 2 side descriptions match
fn match_sides(side1: &HexSides, side2: &HexSides) -> bool {
    //There is probably a better way to do this :(
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
        let mut rotations: Vec<u8> = Vec::new();
        let mut sides = self.sides;
        for i in 0..6 {
            if sides == *matching {
                rotations.push(i as u8);
            }
            sides.rotate_right(1);
        }
        rotations
    }
}
#[derive(Debug, Default, Clone)]
pub struct PossiblitySpace {
    // Vec of possible Hexes for a position, with the possible rotations
    pub possible_hexes: Vec<(Hex, Vec<u8>)>, 
}

impl PossiblitySpace {
    fn trim(&mut self, sides: HexSides) {
        self.possible_hexes.retain(|(hex, _rotations)| match_sides(&hex.sides, &sides));
        //remove rotations
        for (hex, rotations) in self.possible_hexes.iter_mut() {
            *rotations = hex.get_matching_rotations(&sides);
        }
    }
}

#[derive(Debug, Resource)]
pub struct Board {
    vertical_size: usize,
    horizontal_size: usize,
    hexes: Vec<Vec<Hex>>,
    possible_hexes: Vec<Hex>,
    wave_function : Vec<Vec<PossiblitySpace>> //Nice type bro
}

impl Board {
    pub fn new(vertical_size: usize, horizontal_size: usize) -> Board {
        Board {
            vertical_size: vertical_size,
            horizontal_size: horizontal_size,
            hexes: vec![vec![Hex::default(); vertical_size]; horizontal_size],
            possible_hexes: Vec::new(),
            wave_function: vec![vec![PossiblitySpace::default(); vertical_size]; horizontal_size]
        }
    }

    pub fn clear_board(&mut self) {
        self.hexes =  vec![vec![Hex::default(); self.vertical_size]; self.horizontal_size];
        let all_posibilities = self.possible_hexes.iter().map(|hex| (hex.clone(), vec![1,2,3,4,5]));
        self.wave_function =  vec![vec![PossiblitySpace { possible_hexes: all_posibilities.collect() }; self.vertical_size]; self.horizontal_size]

    }

    pub fn get(&self, coordinate: hex2d::Coordinate) -> Option<&Hex> {
        self.hexes
            .get(coordinate.x as usize)?
            .get(coordinate.y as usize)
    }

    pub fn set(&mut self, coordinate: hex2d::Coordinate, hex: Hex) {
        self.hexes[coordinate.x as usize][coordinate.y as usize] = hex.clone();
        // After we set the hex, we propagate the constraints to the sorounding hexes
        for i in coordinate.neighbors() {
            if let Some(direction) = i.direction_to_cw(coordinate) {
                // We find the direction of the neighbour and then use that to set the side socket to the corresponding original side
                self.hexes[i.x as usize][i.y as usize].sides[direction.to_int::<i8>() as usize] =
                    hex.sides[(-direction + Angle::from_int(-(hex.rotation as i8))).to_int::<i8>()
                        as usize];
            }
            // collapse wave function at that coordinate
            self.collapse_wave_function(coordinate);
        }
        
    }

    fn collapse_wave_function(&mut self, coordinate: hex2d::Coordinate) {
        let hex = self.get(coordinate).unwrap().clone();
        let possibility_space: &mut PossiblitySpace = self.wave_function.get_mut(coordinate.x as usize).unwrap().get_mut(coordinate.y as usize).unwrap();
        possibility_space.trim(hex.sides);
        // trim posibility space based on hex perimeter
    }

    pub fn get_possible_hexes_for_coordinate(&self, coordinate: hex2d::Coordinate) -> &PossiblitySpace {
        // If given an invalid coordinate, we just crash :) (HI MEETUP)
        self.wave_function
            .get(coordinate.x as usize).unwrap()
            .get(coordinate.y as usize).unwrap()
    }
    /// Get possible hexes that match the side description
    /// they are not prerotated, so if a hex matches, you need to rotate it in place afterwards
    pub fn get_possible_matching_hexes(&self, sides: &HexSides) -> Vec<&Hex> {
        let filtered: Vec<&Hex> = self
            .possible_hexes
            .iter()
            .filter(|p_hex| {
                match_sides(&p_hex.sides, sides)
            })
            .collect();
        filtered
    }

    pub fn add_possible_hex(&mut self, hex: &Hex) {
        self.possible_hexes.push(hex.clone());
        for row in self.wave_function.iter_mut() {
            for col in row {
                col.possible_hexes.push((hex.clone(), vec![1,2,3,4,5]));
            }
        }
    }

    pub fn reset(&mut self) {
        for i in self.hexes.iter_mut().flatten() {
            *i = Hex::default();
        }
    }
}

#[test]
fn new_board_test() {
    let b = Board::new(3, 3);
    assert_eq!(b.hexes.iter().fold(0, |acc, v| acc + v.len()), 9);
}

#[test]
fn set_hex_test() {
    use hex2d::{Coordinate, Direction::*};
    let mut b = Board::new(3, 3);
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
    let mut b = Board::new(3, 3);
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

    let mut b = Board::new(3, 3);
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
    b.add_possible_hex(&h1);

    let center = Coordinate::new(1, 1);
    b.set(center, h1);
    assert_eq!(b.get(center + YZ).unwrap().sides[3], Side::Grass);
    assert_eq!(
        core::mem::discriminant(&b.get(center + YZ).unwrap().sides[3]),
        core::mem::discriminant(&Side::Grass)
    ); //Any matches everything, so we need to test the actual memory
       //The top side is GRASS, so a match is possible if the hex is rotated, 1,2 or 3 times
    let possible = b.get_possible_hexes_for_coordinate(center + YZ);
    let rotations: Vec<u8> = possible.iter().map(|x| x.rotation).collect();
    assert_eq!(rotations, vec![1, 2, 3]);
}

#[test]
fn possible_matching_test() {
    use hex2d::{Coordinate, Direction::*};
    let mut b = Board::new(3, 3);
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
    b.add_possible_hex(&h1);

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
    let rotations: Vec<u8> = rotations.iter().map(|x| x.rotation).collect();
    assert_eq!(rotations, vec![1, 2, 3]);
}
