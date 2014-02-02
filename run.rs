use agent::Crowd;
use observer::{Observer, ProgressMeter, DiskWriter};

mod agent;
mod geometry;
mod observer;

fn main() {

    let mut cells=Crowd::new(1);
    let tmax=10f64;
    let dt=0.0001;
    let mut t=0.0;

    for _ in range(0,(tmax/dt) as uint){
        let new_cells=cells.evolve(dt);


        let obs_cells=cells; let st=t;
        spawn(proc() {
            let obs = ProgressMeter::new(tmax, 100);
            if obs.request_at(st,dt) {
                obs.see(st, &obs_cells);
            }

            let obs2 = DiskWriter::new(tmax, 50, ~"cells");

            if obs2.request_at(st,dt) {
                obs2.see(st, &obs_cells);
            }
        }
        );

        cells=new_cells;
        t+=dt;
    }
}
