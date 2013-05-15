use agent::*;

mod agent;

// Type Observer
pub trait Observer {
    fn see(&self, time: f64, crowd: &Crowd);
    fn request_at(&self, time: f64, dt: f64) -> bool;
}


//
// * Barre de progression
//
struct ProgressMeter {
    tmax: f64,
    freq: uint
}

pub impl ProgressMeter {
    fn new(tmax: f64, freq: uint) -> ProgressMeter {
        ProgressMeter {tmax: tmax, freq: freq}
    }
}


impl Observer for ProgressMeter {
    fn see(&self, time: f64, crowd: &Crowd) {
        println(fmt!("%f : %u cellules", time as float, crowd.size()));
    }

    fn request_at(&self, time: f64, dt: f64) -> bool {
        let inter_size = self.tmax / (self.freq as f64);
        let int_num=(time/inter_size) as uint;
        (time-inter_size*(int_num as f64)).abs() < dt
    }
}
//
// * Affichage à l'écran
//
struct ScreenPrinter {
    tmax: f64,
    freq: uint
}

pub impl ScreenPrinter {
    fn new(tmax: f64, freq: uint) -> ScreenPrinter {
        ScreenPrinter { tmax: tmax, freq: freq}
    }
}

impl Observer for ScreenPrinter {
    fn see(&self, time: f64, crowd: &Crowd) {
        println(fmt!("%f\n", time as float));
        println(crowd.to_str());
    }

    fn request_at(&self, time: f64, dt: f64) -> bool {
        let inter_size = self.tmax / (self.freq as f64);
        let int_num=(time/inter_size) as uint;
        (time-inter_size*(int_num as f64)).abs() < dt
    }
}

//
// * Sortie sur le disque
//
struct DiskWriter {
    tmax: f64,
    freq: uint,
    fname: ~str,
}

pub impl DiskWriter {
    fn new(tmax: f64, freq:uint, name: ~str) -> DiskWriter {
        DiskWriter {tmax: tmax, freq: freq, fname: copy name}
    }

    fn new_filename(&self, time: f64) -> ~str {
        let inter_size = self.tmax / (self.freq as f64);
        let int_num=(time/inter_size) as uint;
        fmt!("./%s-%03u.vtk", self.fname, int_num)
    }
}

impl Observer for DiskWriter {
    fn see(&self, time: f64, crowd: &Crowd) {
        let outfile=self.new_filename(time);
        let writer = result::get( &io::file_writer( &Path(outfile), [io::Create, io::Truncate] ) );

        // Ecriture de l'entête
        writer.write_str("# vtk DataFile Version 3.0\n");
        writer.write_str("Rusty_Cells\n");
        writer.write_str("ASCII\n");
        writer.write_str("DATASET UNSTRUCTURED_GRID\n");
        writer.write_str(fmt!("POINTS %u float\n", crowd.size()));


        // Coordonnées des cellules
        for crowd.cells.each |&cell| {
            writer.write_str(fmt!("%f %f 0\n",cell.x() as float, cell.y() as float))
        }

        // Données sur les nœuds
        // * Age
        writer.write_str(fmt!("POINT_DATA %u\nSCALARS %s float\nLOOKUP_TABLE default\n", crowd.size(), "age"));
        for crowd.cells.each |&cell| {
            writer.write_str(fmt!("%f\n", cell.age as float));
        }
        // * Vitesse
        writer.write_str(fmt!("VECTORS %s float\n",  "vitesse"));
        for crowd.cells.each |&cell| {
            let speed=cell.velocity*1e3f64;
            writer.write_str(fmt!("%f %f 0\n", speed.x as float, speed.y as float))
        }
    }

    fn request_at(&self, time: f64, dt: f64) -> bool {
        let inter_size = self.tmax / (self.freq as f64);
        let int_num=(time/inter_size) as uint;
        (time-inter_size*(int_num as f64)).abs() < dt
    }
}
