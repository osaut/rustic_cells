
use geometry::Point;
use core::rand::RngUtil;
mod geometry;

//
// Cellule
//

struct Cell {
    center: Point,
    id: uint,
    radius: float,
    velocity: [float,..3],
    acc: [float,..3],
    generation: int,
    age: int
}

pub impl Cell {
    fn new(ident: uint) -> Cell {
        let radius = 1.0;
        //let r=rand::random::<float>();
        //io::println(fmt!("%f\n", r));
        let center : Point =rand::random();

        Cell{ center: center, id: ident, radius: radius, velocity: [0.0,0.0,0.0], acc: [0.0,0.0,0.0], generation: 0, age:0}
    }

    fn move(&self, tumeur: ~Crowd, dt: float) {
        io::println(fmt!("%d @ %f\n",tumeur.size() as int, dt));
    }

    fn replicate(&mut self, tumeur: ~Crowd, dt: float) {
        io::println(fmt!("%d @ %f\n",tumeur.size() as int, dt));

        self.age+=1;
    }
}

impl ToStr for Cell {
    fn to_str(&self) -> ~str {
        fmt!("%d @ (%f, %f)", self.id as int, self.center.x as float, self.center.y as float)
    }
}

//
// Population
//

struct Crowd {
    cells : ~[@Cell],
    time : float
}

pub impl Crowd {
    fn new(init_pop : uint) -> Crowd {
        io::println(fmt!("Population initiale : %d\n",init_pop as int));

        let mut pop : ~[@Cell]=~[];
        for uint::range(1,init_pop) |num| {
            let lonely_one = @Cell::new(num);
            io::println(lonely_one.to_str());
            pop.push(copy lonely_one);
        }
        Crowd{ cells : pop, time: 0.0}
    }

    fn size(&self) -> uint {
        self.cells.len()
    }
}
