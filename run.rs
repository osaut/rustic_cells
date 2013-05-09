use agent::*;
use observer::*;

mod agent;
mod geometry;
mod observer;

fn main() {

    let mut cells=Crowd::new(100);

    let mut obs = ProgressMeter::new(200);
    let mut obs2 = DiskWriter::new(200, ~"cells");
    let dt=0.001;
    let mut t=0.0;
    for 10000.times {
        cells.evolve(dt);
        if(obs.request_at(t)) {
            obs.see(t, &cells);
        }
        if(obs2.request_at(t)) {
            obs2.see(t, &cells);
        }
        t+=dt;
    }
}
