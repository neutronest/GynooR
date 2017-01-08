use std::f64;
use std::f64::NAN;
use std::ops::{Add, Sub, Div, Mul};


// ===== GVector =====

#[derive(Debug, Copy, Clone)]
pub struct GVector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl GVector {
    #[inline]
    pub fn has_nans(&self) -> bool {
        return (self.x == NAN) || (self.y == NAN) || (self.z == NAN);
    }

    #[inline]
    pub fn dot(gv1: GVector, gv2: GVector) -> f64 {
        return gv1.x * gv2.x + gv1.y * gv2.y + gv1.z * gv2.z;
    }

    #[inline]
    pub fn cross_dos(gv1: GVector, gv2: GVector) -> GVector {
        GVector {
            x: (gv1.y * gv2.z) - (gv1.z * gv2.y),
            y: (gv1.z * gv2.x) - (gv1.x * gv2.z),
            z: (gv1.x * gv2.y) - (gv1.y * gv2.x)
        }
    }

    #[inline]
    pub fn length_sqr(&self)  -> f64{
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    #[inline]
    pub fn length(&self) -> f64 {
        return self.length_sqr().sqrt();
    }

    #[inline]
    pub fn normalize(&mut self) -> GVector {
        let len = self.length();
        self.x = self.x / len;
        self.y = self.y / len;
        self.z = self.z / len;
        return self.clone();
    }

    // TODO: CoordinateSystem
    #[inline]
    pub fn coordinate_system(gv1: &mut GVector,
                             gv2: &mut GVector,
                             gv3: &mut GVector) {
        return;
    }


}

// ----- operation overload for (GVector, GVector) 
// TODO: IF SOMEONE KNOW HOW TO USE MACRO LIKE add_impl!
// TO REPLACE THESE STUPID CODE
// FELL FREE TO TEACH ME!!

impl Add<GVector> for GVector {

    type Output = GVector;
    #[inline]
    fn add(self, other: GVector) -> GVector {
        GVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
impl Sub<GVector> for GVector {

    type Output = GVector;
    #[inline]
    fn sub(self, other: GVector) -> GVector {
        GVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    } 
}

impl Mul<GVector> for GVector {

    type Output = GVector;
    #[inline]
    fn mul(self, other: GVector) -> GVector {
        GVector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }

}

impl Div<GVector> for GVector {

    type Output = GVector;
    #[inline]
    fn div(self, other: GVector) -> GVector {
        GVector {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

// ----- operation overload for (GVector, scala) 

impl Add<f64> for GVector {

    type Output = GVector;
    #[inline]
    fn add(self, v: f64) -> GVector {
        GVector {
            x: self.x + v,
            y: self.y + v,
            z: self.z + v
        }
    }
}
impl Sub<f64> for GVector {

    type Output = GVector;
    fn sub(self, v: f64) -> GVector {
        GVector {
            x: self.x - v,
            y: self.y - v,
            z: self.z - v
        }
    } 
}

impl Mul<f64> for GVector {

    type Output = GVector;
    fn mul(self, v: f64) -> GVector {
        GVector {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v
        }
    }

}

impl Div<f64> for GVector {

    type Output = GVector;
    fn div(self, v: f64) -> GVector {
        GVector {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v
        }
    }
}

// ===== GPoint
#[derive(Debug, Copy, Clone)]
pub struct GPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl GPoint {

    #[inline]
    pub fn length_sqr(&self) -> f64{
        return self.x + self.x + self.y * self.y + self.z * self.z;
    }

    #[inline]
    pub fn length(&self) -> f64 {
        return self.length_sqr().sqrt();
    }

    #[inline]
    pub fn distance(gp1: GPoint, gp2: GPoint) -> f64 {
        return (gp1 - gp2).length();
    }

    #[inline]
    pub fn distance_sqr(gp1: GPoint, gp2: GPoint) -> f64 {
        return (gp1 - gp2).length_sqr();
    }
}

// ----- operation overload for (GPoint, GPoint) 

impl Add<GPoint> for GPoint {

    type Output = GPoint;
    #[inline]
    fn add(self, other: GPoint) -> GPoint {
        GPoint {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
impl Sub<GPoint> for GPoint {

    type Output = GPoint;
    #[inline]
    fn sub(self, other: GPoint) -> GPoint {
        GPoint {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    } 
}

impl Mul<GPoint> for GPoint {

    type Output = GPoint;
    #[inline]
    fn mul(self, other: GPoint) -> GPoint {
        GPoint {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }

}

impl Div<GPoint> for GPoint {

    type Output = GPoint;
    #[inline]
    fn div(self, other: GPoint) -> GPoint {
        GPoint {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

// ----- operation overload for (GPoint, scala) 

impl Add<f64> for GPoint {

    type Output = GPoint;
    #[inline]
    fn add(self, v: f64) -> GPoint {
        GPoint {
            x: self.x + v,
            y: self.y + v,
            z: self.z + v
        }
    }
}
impl Sub<f64> for GPoint {

    type Output = GPoint;
    #[inline]
    fn sub(self, v: f64) -> GPoint {
        GPoint {
            x: self.x - v,
            y: self.y - v,
            z: self.z - v
        }
    } 
}

impl Mul<f64> for GPoint {

    type Output = GPoint;
    #[inline]
    fn mul(self, v: f64) -> GPoint {
        GPoint {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v
        }
    }

}

impl Div<f64> for GPoint {

    type Output = GPoint;
    #[inline]
    fn div(self, v: f64) -> GPoint {
        GPoint {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v
        }
    }
}
