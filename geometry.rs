extern mod std;
use std::rand;
use std::rand::{Rand,Rng};
use std::ops;
use std::num::{pow,sqrt};

#[deriving(Clone,Eq)]
pub struct Point {
    x : f64,
    y : f64,
    z : f64
}

impl Point {
    pub fn new() -> Point {
        Point{x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn new_dir() -> Point {
        let mut pt : Point =rand::random();
        pt=pt-0.5f64;
        pt/pt.norm2()
    }

    pub fn norm2(&self) -> f64 {
        sqrt(pow(self.x,2.0)+pow(self.y,2.0)+pow(self.z,2.0))
    }

    pub fn dist(&self, other : &Point) -> f64 {
        (*self-*other).norm2()
    }

}


impl ToStr for Point {
    fn to_str(&self) -> ~str {
        format!("({:f}, {:f}, {:f})", self.x, self.y, self.z)
    }
}

//
// * Génération aléatoire
//
impl Rand for Point {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Point {
        Point { x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: rng.gen::<f64>()}
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
    Point{x: (self.x+lhs.x), y:(self.y+lhs.y), z:(self.z+lhs.z)}
   }
}
impl RhsOfAdd<Point> for f64 {
  fn add_lhs_to(&self, lhs: &Point) -> Point {
    Point{x: (*self+lhs.x), y:(*self+lhs.y), z:(*self+lhs.z)}
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
    Point{x: (-self.x+lhs.x), y:(-self.y+lhs.y), z:(-self.z+lhs.z)}
   }
}
impl RhsOfSub<Point> for f64 {
    fn sub_lhs_to(&self, lhs:&Point) -> Point {
        Point{x: lhs.x - *self, y: lhs.y-*self, z: lhs.z-*self}
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
    Point{x: (self.x*lhs.x), y:(self.y*lhs.y), z:(self.z*lhs.z)}
   }
}
impl RhsOfMul<Point> for f64 {
  fn mul_lhs_to(&self, lhs: &Point) -> Point {
    Point{x: (self*lhs.x), y:(self*lhs.y), z:(self*lhs.z)}
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
    Point{x: (lhs.x/self.x), y:(lhs.y/self.y), z:(lhs.z/self.z)}
   }
}
impl RhsOfDiv<Point> for f64 {
  fn div_lhs_to(&self, lhs: &Point) -> Point {
    Point{x: (lhs.x/ *self), y:(lhs.y/ *self), z:(lhs.z/ *self)}
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
fn test_norm2() {
    let pt1=@Point{ x: 1.0, y: 2.0, z: 1.0};
    let norm=pt1.norm2();

    assert!(norm==sqrt(6.0));
}


#[test]
fn test_dist() {
    let pt1=@Point{ x: 1.0, y: 2.0, z: 0.0};
    let pt2=@Point{ x: 2.0, y: 3.0, z: 1.0};
    let distance=pt1.dist(pt2);

    assert!(distance==sqrt(3.0));
}

#[test]
fn test_add() {
    let pt1=@Point{ x: 1.0, y: 2.0, z: 1.0};
    let pt2=@Point{ x: -1.0, y: 3.0, z: 0.0};
    let pt3=*pt1+*pt2;

    assert!((pt3.x==0.0)&&(pt3.y==5.0)&&(pt3.z==1.0));
}

#[test]
fn test_add_float() {
    let pt1=Point{ x: 1.0, y: 2.0, z: -1.0};
    let pt3=pt1+1.0f64;

    assert!((pt3.x==2.0)&&(pt3.y==3.0)&&(pt3.z==0.0));
}

#[test]
fn test_sub() {
    let pt1=@Point{ x: 1.0, y: 2.0, z: 0.0};
    let pt2=@Point{ x: -1.0, y: 3.0, z: -1.0};
    let pt3=*pt1-*pt2;
    assert!((pt3.x==2.0)&&(pt3.y==-1.0)&&(pt3.z==1.0));
}
fn test_sub_float() {
    let pt1=Point{ x: 1.0, y: 2.0,z: 0.0};
    let pt3=pt1-0.5 as f64;
    assert!((pt3.x==0.5)&&(pt3.y==1.5)&&(pt3.z==-0.5));
}

#[test]
fn test_mult() {
    let pt1=@Point{ x: 1.0, y: 2.0, z: 1.0};
    let pt2=@Point{ x: -1.0, y: 3.0, z: 0.0};
    let pt3=(*pt1)*(*pt2);
    assert!((pt3.x==-1.0)&&(pt3.y==6.0)&&(pt3.z==0.0));
    let pt4=(*pt1)*(2.0 as f64);
    assert!((pt4.x==2.0)&&(pt4.y==4.0)&&(pt4.z==2.0));
}

#[test]
fn test_div() {
    let pt1=@Point{ x: 1.0, y: 2.0, z: 2.0};
    let pt2=@Point{ x: -1.0, y: 3.0, z: -2.0};
    let pt3=(*pt1)/(*pt2);
    assert!(pt3.dist(&Point{x:-1.0, y:2.0/3.0, z: -1.0})==0.0);
    let pt4=(*pt1)/(2.0 as f64);
    assert!((pt4.x==0.5)&&(pt4.y==1.0)&&(pt4.z==1.0));
}

#[test]
fn test_new_dir() {
    let pt = Point::new_dir();
    assert!((pt.norm2()-1.0).abs()<=1e-15f64);
}

#[test]
fn test_rand() {
    let pt1 : Point = rand::random();
    let pt2 : Point = rand::random();
    let pt3 : Point = rand::random();


    assert!(pt1 != pt2); assert!(pt1 != pt3); assert!(pt2 != pt3);
    assert!((pt1.x>=0.0)&&(pt1.y<=1.0)&&(pt1.z<=1.0));
}
