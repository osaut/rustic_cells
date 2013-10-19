
extern mod extra;

use geometry::Point;
use std::rand;
use std::rand::{Rand,rng};
use std::num::{pow,ln};
use std::f64;
use std::uint;
mod geometry;

//
// Cellule
//
#[deriving(Clone)]
struct Cell {
    center: Point,
    id: uint,
    radius: f64,
    velocity: Point,
    acc: Point,
    generation: uint,
    age: f64,
    t_dup: f64
}

impl Cell {
    pub fn new(ident: uint, t_curr: f64) -> Cell {
        let radius = 5e-5f64;
        let center : Point =rand::random();

        Cell{ center: center, id: ident, radius: radius, velocity: Point::new(), acc: Point::new(), generation: 0, age:0f64, t_dup: t_curr+calc_dup_time()}
    }

    pub fn move(&self, tumeur: &~[~Cell], dt: f64) -> ~Cell {

        // Nouvelle position
        let new_center=self.center+self.velocity+self.acc*(0.5*pow(dt,2.0) as f64);

        // Nouvelle accélération
        let new_acc=self.calc_forces(tumeur)+Point::new_dir()*1e-5f64;


        // Nouvelle vitesse ( + frottement)
        let lambda : f64=50.0;
        let denom : f64 = 1.0/(1.0+lambda*dt/2.0);
        let new_velocity=(self.velocity*(1.0-lambda*dt/2.0)+(self.acc+new_acc)*dt/(2.0 as f64))*denom;

        ~Cell{center: new_center, id: self.id, radius: self.radius, velocity: new_velocity, acc: new_acc, generation: self.generation, age: self.age+dt, t_dup: self.t_dup}
    }

    pub fn replicate(&self, tumeur: &~[~Cell], time: f64) -> Option<~Cell> {
        if(self.t_dup<=time) {
            let new_center=self.center + Point::new_dir()*2.2f64*self.radius;
            Some(~Cell{center: new_center, id: tumeur.len()+1, radius: self.radius, velocity: Point::new(), acc: Point::new(), generation: self.generation+1, age:0f64, t_dup: time+calc_dup_time() })
        }
        else {
            None
        }
    }

    fn calc_forces(&self, tumeur: &~[~Cell]) -> Point {
        let seuil=3.0*self.radius;
        let mut force = Point::new();
        for &cell in tumeur.iter() {
            if(cell.id != self.id) {
                let dist_cells=f64::max((self.dist(cell)-2.0*self.radius),0.0f64)+1e-6f64;
                if(dist_cells <= 20f64*self.radius) {
                   let factor_rep =  1.0/pow(dist_cells/seuil,3.0)*1e-7f64 ;
                    let factor_attract = -1.0/pow(dist_cells/(3.0*seuil),2.0)*1e-7f64;
                    force = force + (self.center-cell.center)*(factor_rep+factor_attract);
                }
            }
        }
        force
    }

    pub fn should_die(&self) -> bool {
        self.age > 5.0f64+rand::random::<f64>()
    }

    #[inline(always)]
    pub fn x(&self) -> f64 {
        self.center.x
    }

    #[inline(always)]
    pub fn y(&self) -> f64 {
        self.center.y
    }
    #[inline(always)]
    pub fn z(&self) -> f64 {
        self.center.z
    }


    pub fn dist(&self, other: &Cell) -> f64 {
        self.center.dist(&other.center)
    }

}

impl ToStr for Cell {
    fn to_str(&self) -> ~str {
        format!("{:u} @ ({:f}, {:f}, {:f})", self.id, self.center.x, self.center.y, self.z())
    }
}


/// Calcul du temps de mutation
fn calc_dup_time() -> f64 {
    let lambda=1.0/(2.0*ln(2f64));
    let ord=rand::random::<f64>();

    0.2+ln(1f64-ord)/(-lambda)
}

#[test]
fn test_dup_time() {
    let dup1=calc_dup_time();
    assert!(dup1 >= 0.0f64);
    let dup2=calc_dup_time();
    assert!(dup2!=dup1);
}


#[test]
fn test_new() {
    let cell1=Cell::new(0,0f64);
    let cell2=Cell::new(1,0f64);
    assert!(cell1.center != cell2.center);
}

//
// Population
//

struct Crowd {
    cells : ~[~Cell],
    time : f64
}

impl Crowd {
    pub fn new(init_pop : uint) -> Crowd {

        let mut pop : ~[~Cell]=~[];
        for num in range(1,init_pop+1) {
            let lonely_one = ~Cell::new(num, 0.0f64);
            pop.push(lonely_one);
        }
        Crowd{ cells : pop, time: 0.0}
    }

    pub fn size(&self) -> uint {
        self.cells.len()
    }

    pub fn evolve(&self, dt: f64) -> Crowd {
        let mut new_crowd : ~[~Cell] = ~[];
        for cell in self.cells.iter() {
            if(!cell.should_die()) {

                // Prolifération
                match cell.replicate(&self.cells, self.time) {
                    None => {new_crowd.push(cell.move(&self.cells, dt));}, // Mouvement
                    Some(new_born) => {
                        new_crowd.push(new_born);
                        new_crowd.push(~Cell{center: cell.center, id: cell.id, radius: cell.radius, velocity: cell.velocity, acc: cell.acc, generation: cell.generation, age: cell.age+dt, t_dup: self.time+calc_dup_time()});
                    }
                }

            }
        }

        Crowd {cells: new_crowd, time: self.time+dt}
    }
}


impl ToStr for Crowd {
    fn to_str(&self) -> ~str {
        let mut desc =~"";
        for cell in self.cells.iter() {
            let mut lcell_desc=~"";
            lcell_desc.push_str(cell.to_str()); lcell_desc.push_str("\n");
            desc.push_str(lcell_desc);
        }
        desc
    }
}


#[test]
fn test_size() {
    let crowd=Crowd::new(13);
    assert!(crowd.size()==13);
}
