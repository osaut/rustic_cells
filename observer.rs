use agent::*;

mod agent;


// Type Observer
pub trait Observer {
    fn see(&mut self, time: f64, crowd: &Crowd);
    fn request_at(&mut self, time: f64) -> bool;
}



//
// * Affichage à l'écran
//
struct ScreenPrinter {
    freq: uint,
    ticks: uint
}

pub impl ScreenPrinter {
    fn new(freq: uint) -> ScreenPrinter {
        ScreenPrinter { freq: freq, ticks: 0}
    }
}

impl Observer for ScreenPrinter {
    fn see(&mut self, time: f64, crowd: &Crowd) {
        println(fmt!("%f\n", time as float));
        println(crowd.to_str());
    }

    fn request_at(&mut self, time: f64) -> bool {
        self.ticks+=1;
        self.ticks % self.freq == 0
    }
}

//
// * Sortie sur le disque
//
struct DiskWriter {
    freq: uint,
    fname: ~str,
    curr_snap: uint,
    ticks: uint
}

pub impl DiskWriter {
    fn new(freq:uint, name: ~str) -> DiskWriter {
        DiskWriter {freq: freq, fname: copy name, curr_snap: 0, ticks: 0}
    }

    fn new_filename(&mut self) -> ~str {
        self.curr_snap+=1;
        fmt!("./%s-%u.vtk", self.fname, self.curr_snap-1)
    }
}

impl Observer for DiskWriter {
    fn see(&mut self, time: f64, crowd: &Crowd) {
        let outfile=self.new_filename();
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
        writer.write_str(fmt!("POINT_DATA %u\nSCALARS %s float\nLOOKUP_TABLE default\n", crowd.size(), self.fname));
        for crowd.cells.each |&cell| {
            writer.write_str(fmt!("%u\n", cell.id));
        }
    }

    fn request_at(&mut self, time: f64) -> bool {
        self.ticks+=1;
        self.ticks % self.freq == 0
    }

}
