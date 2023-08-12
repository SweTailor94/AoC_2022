// model types for Day18

use std::collections::HashSet;
use itertools::Itertools;
use crate::input::InputParser;

// By chatGPT:
pub fn surface_area(cubes: Vec<(i32, i32, i32)>) -> i32 {
    // Initialize a set to store the cubes
    let mut cube_set = std::collections::HashSet::new();
    for cube in cubes.iter() {
        cube_set.insert(cube);
    }

    // Initialize a counter for the total surface area
    let mut total_surface_area = 0;

    // Iterate over the cubes
    for (x, y, z) in cubes.iter() {
        // Initialize counters for the number of exposed sides
        let mut exposed_sides_x = 0;
        let mut exposed_sides_y = 0;
        let mut exposed_sides_z = 0;

        // Check each of the six neighbors
        for (dx, dy, dz) in [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)].iter() {
            // If the neighbor is not in the set, then the cube has an exposed side on that face
            if !cube_set.contains(&(x + dx, y + dy, z + dz)) {
                if *dx != 0 {
                    exposed_sides_x += 1;
                }
                if *dy != 0 {
                    exposed_sides_y += 1;
                }
                if *dz != 0 {
                    exposed_sides_z += 1;
                }
            }
        }

        // Add the surface area for this cube to the total
        total_surface_area += exposed_sides_x + exposed_sides_y + exposed_sides_z;
    }

    total_surface_area
}

pub fn another_try(cubes: Vec<(i32, i32, i32)>) -> i32 {
    // Parse the input


    let mut surface_area = 0;

    // Convert the list of cubes to a set for fast lookup
    let cubes_set: HashSet<(i32, i32, i32)> = cubes.iter().cloned().collect();

    // Iterate over the cubes and count the number of exposed faces
    for &(x, y, z) in &cubes {
        if !cubes_set.contains(&(x + 1, y, z)) {
            surface_area += 1;
        }
        if !cubes_set.contains(&(x - 1, y, z)) {
            surface_area += 1;
        }
        if !cubes_set.contains(&(x, y + 1, z)) {
            surface_area += 1;
        }
        if !cubes_set.contains(&(x, y - 1, z)) {
            surface_area += 1;
        }
        if !cubes_set.contains(&(x, y, z + 1)) {
            surface_area += 1;
        }
        if !cubes_set.contains(&(x, y, z - 1)) {
            surface_area += 1;
        }
    }

    surface_area
}

// fn main() {
//     // Parse the input
//     let cubes = vec![        (2, 2, 2),        (1, 2, 2),        (3, 2, 2),        (2, 1, 2),        (2, 3, 2),        (2, 2, 1),        (2, 2, 3),        (2, 2, 4),        (2, 2, 6),        (1, 2, 5),        (3, 2, 5),        (2, 1, 5),        (2, 3, 5),    ];
//
//     let mut surface_area = 0;
//
//     // Convert the list of cubes to a set for fast lookup
//     let cubes_set: HashSet<(i32, i32, i32)> = cubes.iter().cloned().collect();
//
//     // Iterate over the cubes and count the number of exposed faces
//     for &(x, y, z) in &cubes {
//         if !cubes_set.contains(&(x+1, y, z)) && !cubes_set.contains(&(x+1, y, z+1)) && !cubes_set.contains(&(x+1, y, z-1)) && !cubes_set.contains(&(x+1, y+1, z)) && !cubes_set.contains(&(x+1, y-1, z)) {
//             surface_area += 1;
//         }
//         if !cubes_set.contains(&(x-1, y, z)) && !cubes_set.contains(&(x-1, y, z+1)) && !cubes_set.contains(&(x-1, y, z-1)) && !cubes_set.contains(&(x-1, y+1, z)) && !cubes_set.contains(&(x-1, y-1, z)) {
//             surface_area += 1;
//         }
//         if !cubes_set.contains(&(x, y+1, z)) && !cubes_set.contains(&(x, y+1, z+1)) && !cubes_set.contains(&(x, y+1, z-1)) && !cubes_set.contains(&(x+1, y+1, z)) && !cubes_set.contains(&(x-1, y+1, z)) {
//             surface_area += 1;
//         }
//         if !cubes_set.contains(&(x, y-1, z)) && !cubes_set.contains(&(x, y-1, z+1)) && !cubes_set.contains(&(x, y-1, z-1)) && !cubes_set.contains(&(x+1, y-1, z)) && !cubes_set.contains(&(x-1, y-1, z)) {
//             surface_area += 1;
//         }
//         if !cubes_set.contains(&(x, y, z+1)) && !cubes_set.contains(&(x, y+1, z+1)) && !cubes_set


const CUBE_FACES: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];


pub struct Droplet {
    cubes: HashSet<(i32, i32, i32)>,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl InputParser for Droplet {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        let xyz = line.split(",").map(|v| v.parse::<i32>().unwrap()).collect_vec();
        self.add_cube(xyz[0], xyz[1], xyz[2]);
        if self.x_min > xyz[0] { self.x_min = xyz[0] };
        if self.x_max < xyz[0] { self.x_max = xyz[0] };
        if self.y_min > xyz[1] { self.y_min = xyz[1] };
        if self.y_max < xyz[1] { self.y_max = xyz[1] };
        if self.z_min > xyz[2] { self.z_min = xyz[2] };
        if self.z_max < xyz[2] { self.z_max = xyz[2] };
        Ok(())
    }
}

impl Droplet {
    pub fn new() -> Droplet {
        Droplet {
            cubes: HashSet::new(),
            x_min: i32::MAX,
            x_max: i32::MIN,
            y_min: i32::MAX,
            y_max: i32::MIN,
            z_min: i32::MAX,
            z_max: i32::MIN,

        }
    }

    pub fn add_cube(&mut self, x: i32, y: i32, z: i32) {
        self.cubes.insert((x, y, z));
    }

    pub fn surface_area(&self) -> i32 {
        let mut surface_area = 0;
        for (x, y, z) in &self.cubes {
            for (dx, dy, dz) in &CUBE_FACES {
                if !self.cubes.contains(&(*x + dx, *y + dy, *z + dz)) {
                    surface_area += 1;
                }
            }
        }
        surface_area
    }

    pub fn exterior_surface_area(&self) -> i32 {
        let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
        let mut exterior_surface_area = self.surface_area();
        println!("x: {}..={}", self.x_min, self.x_max);
        println!("y: {}..={}", self.y_min, self.y_max);
        println!("z: {}..={}", self.z_min, self.z_max);
        // Build vector of empty blocks
        let mut empty: HashSet<(i32, i32, i32)> = HashSet::new();

        for x in self.x_min..=self.x_max {
            for y in self.y_min..=self.y_max {
                for z in self.z_min..=self.z_max {
                    let block = (x, y, z);
                    if !self.cubes.contains(&block) {
                        empty.insert(block);
                    }
                }
            }
        }
        println!("Total {}", (self.x_max - self.x_min + 1) * (self.y_max - self.y_min + 1) * (self.z_max - self.z_min + 1));
        println!("cubes {}, empty {} sum {}", self.cubes.len(), empty.len(), self.cubes.len() + empty.len());

        // remove all empty that are connected to the outside
        //let mut to_remove: HashSet::<(i32, i32, i32)> = HashSet::new();

        // For each empty on the edge, find all connected empty (similar to finding connected blocks
        let edge_empty = empty.iter().filter(|(x,y,z)|
                                                 *x == self.x_min || *x == self.x_max ||
                                                 *y == self.y_min || *y == self.y_max ||
                                                 *z == self.z_min || *z == self.z_max ).collect_vec();
        for (x, y, z) in edge_empty {
            if visited.contains(&(*x, *y, *z)) {
                continue;
            }
            let mut stack = Vec::new();
            stack.push((*x, *y, *z));
            while let Some((x, y, z)) = stack.pop() {
                if  !visited.contains(&(x, y, z)) && empty.contains(&(x, y, z)) {
                    visited.insert((x, y, z));

                    for (dx, dy, dz) in &CUBE_FACES {
                        let xi = x + dx ;
                        let yi = y + dy;
                        let zi = z + dz;
                        if xi > self.x_max || xi < self.x_min  ||
                            yi > self.y_max  || yi < self.y_min  ||
                            zi > self.z_max  || zi < self.z_min  {
                            continue;
                        }
                        stack.push((xi, yi, zi));
                    }
                }
            }

        }
        // remove all empty that are connected to the outside
        for e in visited.iter() {
            empty.remove(e);
        }

        let mut interior_surface_area = 0;
        for (x, y, z) in empty.iter() {
            for (dx, dy, dz) in &CUBE_FACES {
                if !empty.contains(&(*x + dx, *y + dy, *z + dz)) {
                    interior_surface_area += 1;
                }
            }
        }

        exterior_surface_area - interior_surface_area
    }
}