
use geometry::Point;
use core::rand::RngUtil;
use core::float::pow;
mod geometry;

//
// Cellule
//

struct Cell {
    center: Point,
    id: uint,
    radius: f64,
    velocity: Point,
    acc: Point,
    generation: int,
    age: int
}

pub impl Cell {
    fn new(ident: uint) -> Cell {
        let radius = 1.0;
        //let r=rand::random::<float>();
        //io::println(fmt!("%f\n", r));
        let center : Point =rand::random();

        Cell{ center: center, id: ident, radius: radius, velocity: Point::new(), acc: Point::new(), generation: 0, age:0}
    }

    fn move(&mut self, tumeur: &Crowd, dt: f64) {
        let old_acc = copy self.acc;

        // Nouvelle position
        self.center+=self.velocity+self.acc*(0.5*float::pow(dt,2.0) as f64);

        // Force aléatoire
        let F_alea: Point=rand::random();
        self.acc=self.calc_rep_force(tumeur)+F_alea*(0.0005 as f64);


        // Nouvelle vitesse
        let lambda : f64=5.0;
        let denom : f64 = 1.0/(1.0+lambda*dt/2.0);
        self.velocity=(self.velocity*(1.0-lambda*dt/2.0)+(self.acc+old_acc)*dt/(2.0 as f64))*denom;

    }

    fn replicate(&mut self, tumeur: &Crowd, dt: f64) {
        io::println(fmt!("%d @ %f\n",tumeur.size() as int, dt as float));

        self.age+=1;
    }

    fn calc_rep_force(&self, tumeur: &Crowd) -> Point {
        let seuil=3.0*self.radius;
        let mut force = Point::new();
        for tumeur.cells.each |&cell| {
            if(cell.id != self.id) {
                let dist_cells=(self.dist(cell)+2.0*self.radius)/seuil;
                let factor = 1.0/pow(dist_cells,3.0)*1e-7 as f64;
                force += (self.center-cell.center)*factor;
            }
        }
        force
    }

    fn dist(&self, other: &Cell) -> f64 {
        self.center.dist(&other.center)
    }

}

impl ToStr for Cell {
    fn to_str(&self) -> ~str {
        fmt!("%d @ (%f, %f)", self.id as int, self.center.x as float, self.center.y as float)
    }
}

#[test]
fn test_new() {
    let cell1=Cell::new(0);
    let cell2=Cell::new(1);
    assert!(cell1.center != cell2.center);
}
//
// Population
//

struct Crowd {
    cells : ~[@mut Cell],
    time : float
}

pub impl Crowd {
    fn new(init_pop : uint) -> Crowd {

        let mut pop : ~[@mut Cell]=~[];
        for uint::range(1,init_pop+1) |num| {
            let lonely_one = @mut Cell::new(num);
            pop.push(copy lonely_one);
        }
        Crowd{ cells : pop, time: 0.0}
    }

    fn size(&self) -> uint {
        self.cells.len()
    }

    fn evolve(&self, dt: f64) {
        for self.cells.each |cell| {
            cell.move(self, dt);
        }
    }
}


impl ToStr for Crowd {
    fn to_str(&self) -> ~str {
        let mut desc =~"";
        for self.cells.each |cell| {
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
