//! Monte-Carlo simulation of a binary mixture of H20 and CO2.
extern crate cymbalum;
use cymbalum::*;

fn main() {
    Logger::stdout();

    let mut universe = Universe::from_file("data/binary.xyz").unwrap();

    // Add bonds in the system
    for i in 0..universe.molecules().len() / 3 {
        universe.add_bond(3 * i,     3 * i + 1);
        universe.add_bond(3 * i + 1, 3 * i + 2);
    }

    universe.set_cell(UnitCell::cubic(25.0));
    input::read_interactions(&mut universe, "data/binary.yml").unwrap();

    let co2 = {
        // We can read files to get moltype
        let (molecule, atoms) = input::molecule_from_file("data/CO2.xyz", true).unwrap();
        moltype(&molecule, &atoms)
    };
    let h2o = {
        // Or define a new molecule by hand
        let mut molecule = Molecule::new(0);
        molecule.merge_with(Molecule::new(1));
        molecule.merge_with(Molecule::new(2));

        molecule.add_bond(0, 1);
        molecule.add_bond(1, 2);

        moltype(&molecule, &[Particle::new("H"), Particle::new("O"), Particle::new("H")])
    };

    let mut mc = MonteCarlo::new(units::from(500.0, "K").unwrap());

    // Use the molecular types of CO2 and H2O to specify different probabilities
    mc.add(Box::new(Translate::with_moltype(units::from(0.5, "A").unwrap(), co2)), 1.0);
    mc.add(Box::new(Rotate::with_moltype(units::from(10.0, "deg").unwrap(), co2)), 1.0);

    mc.add(Box::new(Translate::with_moltype(units::from(10.0, "A").unwrap(), h2o)), 2.0);
    mc.add(Box::new(Rotate::with_moltype(units::from(20.0, "deg").unwrap(), h2o)), 2.0);

    let mut simulation = Simulation::new(mc);
    simulation.run(&mut universe, 200_000_000);
}
