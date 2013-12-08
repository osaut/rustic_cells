use agent::Crowd;
use std::io::fs::File;
use std::io::{Write, Truncate};
mod agent;

// Type Observer
pub trait Observer {
    fn see(&self, time: f64, crowd: &Crowd);
    fn request_at(&self, time: f64, dt: f64) -> bool;
    // fn request_at(&self, time: f64, dt: f64) -> bool {
    //     let inter_size = self.tmax / (self.freq as f64);
    //     let int_num=(time/inter_size) as uint;
    //     (time-inter_size*(int_num as f64)).abs() < dt
    // }
}


//
// * Barre de progression
//
struct ProgressMeter {
    tmax: f64,
    freq: uint
}

impl ProgressMeter {
    pub fn new(tmax: f64, freq: uint) -> ProgressMeter {
        ProgressMeter {tmax: tmax, freq: freq}
    }
}


impl Observer for ProgressMeter {
    fn see(&self, time: f64, crowd: &Crowd) {
        println!("{:f} : {:u} cellules", time, crowd.size());
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

impl ScreenPrinter {
    pub fn new(tmax: f64, freq: uint) -> ScreenPrinter {
        ScreenPrinter { tmax: tmax, freq: freq}
    }
}

impl Observer for ScreenPrinter {
    fn see(&self, time: f64, crowd: &Crowd) {
        println!("{:f}\n", time);
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

impl DiskWriter {
    pub fn new(tmax: f64, freq:uint, name: ~str) -> DiskWriter {
        DiskWriter {tmax: tmax, freq: freq, fname: name}
    }

    pub fn new_filename(&self, time: f64) -> ~str {
        let inter_size = self.tmax / (self.freq as f64);
        let int_num=(time/inter_size) as uint;
        format!("./{:s}-{:03u}.vtk", self.fname, int_num)
    }
}

impl Observer for DiskWriter {
    fn see(&self, time: f64, crowd: &Crowd) {
        let outfile=self.new_filename(time);
        let file_path=Path::new(outfile);

        match File::open_mode(&file_path, Truncate , Write)  {
            None => fail!(),

            Some(ref mut f) => {
                let writer= f as &mut Writer;
                // Ecriture de l'entête
                write!(writer, "{:c} vtk DataFile Version 3.0\n", '#');
                write!(writer, "Rusty_Cells\n");
                write!(writer, "ASCII\n");
                write!(writer, "DATASET UNSTRUCTURED_GRID\n");
                write!(writer, "POINTS {:u} float\n", crowd.size());


                // Coordonnées des cellules
                for cell in crowd.cells.iter() {
                    write!(writer, "{:f} {:f} {:f}\n", cell.x(), cell.y(), cell.z() );
                }

                // Données sur les nœuds
                // * Age
                 write!(writer,"POINT_DATA {:u}\nSCALARS {:s} float\nLOOKUP_TABLE default\n", crowd.size(), "age");
                for cell in crowd.cells.iter() {
                    write!(writer,"{:f}\n", cell.age);
                }
                // * Vitesse
                write!(writer,"VECTORS {:s} float\n",  "vitesse");
                for cell in crowd.cells.iter() {
                    let speed=cell.velocity*1e3f64;
                    write!(writer,"{:f} {:f} {:f}\n", speed.x, speed.y, speed.z)
                }
            }
        }
    }

    fn request_at(&self, time: f64, dt: f64) -> bool {
        let inter_size = self.tmax / (self.freq as f64);
        let int_num=(time/inter_size) as uint;
        (time-inter_size*(int_num as f64)).abs() < dt
    }
}
