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
    fn has_nans(&self) -> bool {
        return (self.x == NAN) || (self.y == NAN) || (self.z == NAN);
    }

    fn dot(gv1: GVector, gv2: GVector) -> f64 {
        return gv1.x * gv2.x + gv1.y * gv2.y + gv1.z * gv2.z;
    }

    fn cross_dos(gv1: GVector, gv2: GVector) -> GVector {
        GVector {
            x: (gv1.y * gv2.z) - (gv1.z * gv2.y),
            y: (gv1.z * gv2.x) - (gv1.x * gv2.z),
            z: (gv1.x * gv2.y) - (gv1.y * gv2.x)
        }
    }
}

// ----- operation overload for (GVector, GVector) 

impl Add<GVector> for GVector {

    type Output = GVector;
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
