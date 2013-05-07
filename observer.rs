use agent::*;

mod agent;


// Type Observer
pub trait Observer {
    fn see(&self, time: f64, crowd: &Crowd);
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
    fn see(&self, time: f64, crowd: &Crowd) {
        io::println(fmt!("%f\n", time as float));
        io::println(crowd.to_str());
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
    ticks: uint
}

pub impl DiskWriter {
    fn new(freq:uint, name: ~str) -> DiskWriter {
        DiskWriter {freq: freq, fname: name, ticks: 0}
    }
}

impl Observer for DiskWriter {
    fn see(&self, time: f64, crowd: &Crowd) {
      io::println(fmt!("%f : %d cells -> %s.\n",time as float, crowd.size() as int, self.fname));
    }

    fn request_at(&mut self, time: f64) -> bool {
        self.ticks+=1;
        self.ticks % self.freq == 0
    }

}
