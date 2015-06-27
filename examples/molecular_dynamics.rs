extern crate cymbalum;
use cymbalum::*;

fn main() {
    let mut universe = Universe::from_cell(UnitCell::cubic(20.0));

    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                let mut part = Particle::new("Ar");
                part.set_position(Vector3D::new(
                        i as f64 * 2.0,
                        j as f64 * 2.0,
                        k as f64 * 2.0
                ));
                universe.add_particle(part);
            }
        }
    }
    universe.add_pair_interaction("Ar", "Ar", LennardJones{sigma: 3.4, epsilon: 1e-4});

    let mut simulation = Simulation::new(MolecularDynamics::new(1.0));
    simulation.add_output(TrajectoryOutput::new("trajectory.xyz"));

    simulation.run(&mut universe, 500);

    println!("All done!")
}
