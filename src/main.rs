pub mod agent;
pub mod network;

fn main() {
    let mut population = agent::random_population(450);

    for _ in 0..1000000 {
        population = agent::generation(population);
    }
}
