// strcat: rusti: trait Foo { fn foo(&self); } trait Bar {}; impl<T: Bar> Foo
// for T { fn foo(&self) { println("default impl") } }; impl Bar for int {};
// 5.foo()

use agent::Crowd;
use std::io;
use std::result;
use std::os;
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
        let writer_result = io::file_writer(~file_path, [io::Create, io::Truncate] );

        if writer_result.is_ok() {
            let writer=writer_result.unwrap();

            // Ecriture de l'entête
            writer.write_str("# vtk DataFile Version 3.0\n");
            writer.write_str("Rusty_Cells\n");
            writer.write_str("ASCII\n");
            writer.write_str("DATASET UNSTRUCTURED_GRID\n");
            writer.write_str(format!("POINTS {:u} float\n", crowd.size()));


            // Coordonnées des cellules
            for cell in crowd.cells.iter() {
                writer.write_str(format!("{:f} {:f} {:f}\n",cell.x(), cell.y(), cell.z()))
            }

            // Données sur les nœuds
            // * Age
            writer.write_str(format!("POINT_DATA {:u}\nSCALARS {:s} float\nLOOKUP_TABLE default\n", crowd.size(), "age"));
            for cell in crowd.cells.iter() {
                writer.write_str(format!("{:f}\n", cell.age));
            }
            // * Vitesse
            writer.write_str(format!("VECTORS {:s} float\n",  "vitesse"));
            for cell in crowd.cells.iter() {
                let speed=cell.velocity*1e3f64;
                writer.write_str(format!("{:f} {:f} {:f}\n", speed.x, speed.y, speed.z))
            }
        }
    }

    fn request_at(&self, time: f64, dt: f64) -> bool {
        let inter_size = self.tmax / (self.freq as f64);
        let int_num=(time/inter_size) as uint;
        (time-inter_size*(int_num as f64)).abs() < dt
    }
}
