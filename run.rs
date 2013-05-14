use agent::*;
use observer::*;

mod agent;
mod geometry;
mod observer;

fn main() {

    let mut cells=Crowd::new(1);

    let mut obs = ProgressMeter::new(200);
    let mut obs2 = DiskWriter::new(200, ~"cells");
    let dt=0.0001;
    let mut t=0.0;
    for 100000.times {
        let new_cells=cells.evolve(dt);
        if(obs.request_at(t)) {
            obs.see(t, &cells);
        }
        if(obs2.request_at(t)) {
            obs2.see(t, &cells);
        }
        cells=new_cells;
        t+=dt;
    }
}
