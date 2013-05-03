use agent::*;

mod agent;


// Type Observer
pub trait Observer {
    fn see(&mut self, time: f64, crowd: ~Crowd);
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
    fn see(&mut self, time: f64, crowd: ~Crowd) {
       self.ticks+=1;
       if (self.ticks % self.freq == 0) {
        io::println(fmt!("%f : %d cells.\n",time as float, crowd.size() as int));
       }
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
    fn see(&mut self, time: f64, crowd: ~Crowd) {
       self.ticks+=1;
       if (self.ticks % self.freq == 0) {
        io::println(fmt!("%f : %d cells -> %s.\n",time as float, crowd.size() as int, self.fname));
       }
    }

}
