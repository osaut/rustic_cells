use core::rand::*;
use core::f64::sqrt;
use core::f64::pow;

#[deriving(Eq)]
pub struct Point {
    x : f64,
    y : f64
}

pub impl Point {
    fn new() -> Point {
        Point{x:0.0, y:0.0}
    }

    fn dist(&self, other : &Point) -> f64 {
        sqrt(pow(self.x-other.x,2.0)+pow(self.y-other.y,2.0))
    }
}


impl ToStr for Point {
    fn to_str(&self) -> ~str {
        fmt!("(%f, %f)", self.x as float, self.y as float)
    }
}

//
// * Génération aléatoire
//
impl Rand for Point {
    fn rand<R: Rng>(rng: &R) -> Point {
        Point { x: rng.gen::<f64>(), y: rng.gen::<f64>()}
    }
}

//
// * Opérateurs arithmétiques
//
// Addition
trait RhsOfAdd<Result> {
  fn add_lhs_to(&self, lhs: &Point) -> Result;
}
impl RhsOfAdd<Point> for Point {
  fn add_lhs_to(&self, lhs: &Point) -> Point {
    Point{x: (self.x+lhs.x), y:(self.y+lhs.y)}
   }
}
impl<Result, Rhs: RhsOfAdd<Result> > ops::Add<Rhs, Result> for Point {
  fn add(&self, rhs: &Rhs) -> Result {
    rhs.add_lhs_to(self)
  }
}
// Soustraction
trait RhsOfSub<Result> {
  fn sub_lhs_to(&self, lhs: &Point) -> Result;
}
impl RhsOfSub<Point> for Point {
  fn sub_lhs_to(&self, lhs: &Point) -> Point {
    Point{x: (-self.x+lhs.x), y:(-self.y+lhs.y)}
   }
}
impl RhsOfSub<Point> for f64 {
    fn sub_lhs_to(&self, lhs:&Point) -> Point {
        Point{x: lhs.x - *self, y: lhs.y-*self}
    }
}
impl<Result, Rhs: RhsOfSub<Result> > ops::Sub<Rhs, Result> for Point {
  fn sub(&self, rhs: &Rhs) -> Result {
    rhs.sub_lhs_to(self)
  }
}
// Multiplication
trait RhsOfMul<Result> {
  fn mul_lhs_to(&self, lhs: &Point) -> Result;
}
impl RhsOfMul<Point> for Point {
  fn mul_lhs_to(&self, lhs: &Point) -> Point {
    Point{x: (self.x*lhs.x), y:(self.y*lhs.y)}
   }
}
impl RhsOfMul<Point> for f64 {
  fn mul_lhs_to(&self, lhs: &Point) -> Point {
    Point{x: (self*lhs.x), y:(self*lhs.y)}
   }
}

impl<Result, Rhs: RhsOfMul<Result> > ops::Mul<Rhs, Result> for Point {
  fn mul(&self, rhs: &Rhs) -> Result {
    rhs.mul_lhs_to(self)
  }
}
// Division
trait RhsOfDiv<Result> {
  fn div_lhs_to(&self, lhs: &Point) -> Result;
}
impl RhsOfDiv<Point> for Point {
  fn div_lhs_to(&self, lhs: &Point) -> Point {
    Point{x: (lhs.x/self.x), y:(lhs.y/self.y)}
   }
}
impl RhsOfDiv<Point> for f64 {
  fn div_lhs_to(&self, lhs: &Point) -> Point {
    Point{x: (lhs.x/ *self), y:(lhs.y/ *self)}
   }
}
impl<Result, Rhs: RhsOfDiv<Result> > ops::Div<Rhs, Result> for Point {
  fn div(&self, rhs: &Rhs) -> Result {
    rhs.div_lhs_to(self)
  }
}

//
// * Test
//
#[test]
fn test_dist() {
    let pt1=@Point{ x: 1.0, y: 2.0};
    let pt2=@Point{ x: 2.0, y: 3.0};
    let distance=pt1.dist(pt2);

    assert!(distance==f64::sqrt(2.0));
}

#[test]
fn test_add() {
    let pt1=@Point{ x: 1.0, y: 2.0};
    let pt2=@Point{ x: -1.0, y: 3.0};
    let pt3=*pt1+*pt2;

    assert!((pt3.x==0.0)&&(pt3.y==5.0));
}

#[test]
fn test_sub() {
    let pt1=@Point{ x: 1.0, y: 2.0};
    let pt2=@Point{ x: -1.0, y: 3.0};
    let pt3=*pt1-*pt2;
    assert!((pt3.x==2.0)&&(pt3.y==-1.0));
}
fn test_sub_float() {
    let pt1=Point{ x: 1.0, y: 2.0};
    let pt3=pt1-0.5 as f64;
    assert!((pt3.x==0.5)&&(pt3.y==1.5));
}

#[test]
fn test_mult() {
    let pt1=@Point{ x: 1.0, y: 2.0};
    let pt2=@Point{ x: -1.0, y: 3.0};
    let pt3=(*pt1)*(*pt2);
    assert!((pt3.x==-1.0)&&(pt3.y==6.0));
    let pt4=(*pt1)*(2.0 as f64);
    assert!((pt4.x==2.0)&&(pt4.y==4.0));
}

#[test]
fn test_div() {
    let pt1=@Point{ x: 1.0, y: 2.0};
    let pt2=@Point{ x: -1.0, y: 3.0};
    let pt3=(*pt1)/(*pt2);
    assert!(pt3.dist(&Point{x:-1.0, y:2.0/3.0})==0.0);
    let pt4=(*pt1)/(2.0 as f64);
    assert!((pt4.x==0.5)&&(pt4.y==1.0));
}

#[test]
fn test_rand() {
    let pt1 : Point = rand::random();
    let pt2 : Point = rand::random();

    assert!(pt1 != pt2);
}
