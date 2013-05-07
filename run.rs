use agent::*;
use observer::*;

mod agent;
mod geometry;
mod observer;

fn main() {

    let cells=Crowd::new(10);
    io::println(cells.to_str());

    let mut obs = ScreenPrinter::new(10);

    let dt=0.001;
    let mut t=0.0;
    for 100.times {
        cells.evolve(dt);
        if(obs.request_at(t)) {
            obs.see(t, &cells);
        }
        t+=dt;
    }
}
