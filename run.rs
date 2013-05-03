use agent::*;

mod agent;
mod geometry;
mod observer;

fn main() {
    let cells=Crowd::new(10);
    io::println(cells.to_str());
    io::println("\n");
    cells.evolve();
    io::println(cells.to_str());
}
