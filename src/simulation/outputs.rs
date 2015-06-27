/*
 * Cymbalum, Molecular Simulation in Rust
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
use std::io::prelude::*;
use std::fs::File;

use ::universe::Universe;

/// Type for data ouptut from a simulation.
pub trait Output {
    /// Function called once at the beggining of the simulation, which allow
    /// for some setup if needed.
    fn setup(&mut self, _: &Universe) {}

    /// Write the output from the universe.
    fn write(&mut self, universe: &Universe);

    /// Function called once at the end of the simulation.
    fn finish(&mut self, _: &Universe) {}
}

/******************************************************************************/

pub struct TrajectoryOutput {
    file: File,
}

impl TrajectoryOutput {
    pub fn new<'a, S>(filename: S) -> TrajectoryOutput where S: Into<&'a str> {
        TrajectoryOutput{
            file: File::create(filename.into()).unwrap()
        }
    }
}

impl Output for TrajectoryOutput {
    fn write(&mut self, universe: &Universe) {
        writeln!(&mut self.file, "{}", universe.size()).unwrap();
        writeln!(&mut self.file, "Generated by cymbalum \\o/").unwrap();
        for n in 0..universe.size() {
            let part = &universe[n];
            let pos = part.position();
            writeln!(&mut self.file, "{} {} {} {}", part.name(), pos.x, pos.y, pos.z).unwrap();
        }
    }
}
