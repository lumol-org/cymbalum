/* Cymbalum, Molecular Simulation in Rust - Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
 */
//! Global potential are potentials acting on the whole system at once
//!
//! They can be coulombic potentials, or external provided potential function
//! for example.
use universe::Universe;
use types::{Matrix3, Vector3D};
use super::PairRestriction;

/// The `GlobalPotential` trait represent a potential acting on the whole
/// universe at once.
pub trait GlobalPotential: GlobalCache + BoxCloneGlobal {
    /// Compute the energetic contribution of this potential
    fn energy(&self, universe: &Universe) -> f64;
    /// Compute the force contribution of this potential
    fn forces(&self, universe: &Universe) -> Vec<Vector3D>;
    /// Compute the virial contribution of this potential
    fn virial(&self, universe: &Universe) -> Matrix3;
}

/// Energetic cache for global potentials. This trait have an opt-in default
/// implementation that you can use by implementing the `DefaultGlobalCache`
/// trait.
///
/// # Example
///
/// ```rust
/// struct MyGlobalPotential;
///
/// impl GlobalPotential for MyGlobalPotential {
///    ... // Implementation
/// }
///
/// // Use the default cache
/// impl DefaultGlobalCache for MyGlobalPotential {}
/// ```
pub trait GlobalCache {
    /// Get the cost of moving the `universe` particles whose indexes are in
    /// `idxes` to `newpos`
    fn move_particles_cost(&self, universe: &Universe, idxes: &[usize], newpos: &[Vector3D]) -> f64;
    /// Update the cache after a call to a `*_cost` function.
    fn update(&mut self);
}

/// Marker trait for the default implementation of GlobalCache. This default
/// implementation is slow, as it create a copy of the universe
pub trait DefaultGlobalCache {}

impl<T> GlobalCache for T where T: DefaultGlobalCache + GlobalPotential {
    fn move_particles_cost(&self, universe: &Universe, idxes: &[usize], newpos: &[Vector3D]) -> f64 {
        let mut universe = universe.clone();
        let old_e = self.energy(&universe);
        for (i, &pi) in idxes.iter().enumerate() {
            universe[pi].position = newpos[i];
        }
        let new_e = self.energy(&universe);
        return new_e - old_e;
    }

    fn update(&mut self) {
        // Nothing to do
    }
}

/// Electrostatic potential solver should implement the `CoulombicPotential`
/// trait.
pub trait CoulombicPotential : GlobalPotential + BoxCloneCoulombic {
    /// Set the restriction scheme to use to `restriction`. All future call to
    /// `energy`, `force` or `virial` should use this restriction.
    fn set_restriction(&mut self, restriction: PairRestriction);
}

impl_box_clone!(GlobalPotential, BoxCloneGlobal, box_clone_gobal);
impl_box_clone!(CoulombicPotential, BoxCloneCoulombic, box_clone_coulombic);

mod wolf;
pub use self::wolf::Wolf;

mod ewald;
pub use self::ewald::Ewald;
