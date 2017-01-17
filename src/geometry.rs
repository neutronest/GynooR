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
    pub fn cross_dot(gv1: GVector, gv2: GVector) -> GVector {
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

// =====

pub struct GRay {

    pub origin: GVector,

    pub direct: GVector
}

impl GRay {

    pub fn get_point(&self, t: f64) -> GVector {
        self.origin + self.direct * t
    }
}

// ===== Sphere
#[derive(Debug, Copy, Clone)]
pub struct GSphere {

    pub center: GVector,

    pub radius: f64

}

impl GSphere {

    pub fn radius_sqr(&self) -> f64 {

        self.radius * self.radius
    }

    pub fn get_intersect(&self, ray: GRay) -> IntersectResult {

        let mut res = IntersectResult::new();

        let v = ray.origin - self.center;
        let a0 = v.length_sqr() - self.radius_sqr();
        let d_dot_v = GVector::dot(ray.direct, v);

        if d_dot_v <= 0.0 {

            let discr = d_dot_v * d_dot_v - a0;
            if discr >= 0.0 {
                res.flag = true;
                res.distance = -1.0 * d_dot_v - discr.sqrt();
                res.position = ray.get_point(res.distance);
                res.normal = (res.position - self.center).normalize();
            }
        }
        return res;

    }



}


// === IntersectResult

pub struct IntersectResult {

    pub flag: bool,

    pub distance: f64,

    pub position: GVector,

    pub normal: GVector
}

impl IntersectResult {

    pub fn new() -> IntersectResult {
        return IntersectResult{flag: false,
                               distance: 0.0,
                               position: GVector{x: 0.0, y: 0.0, z: 0.0},
                               normal: GVector{x: 0.0, y: 0.0, z:0.0}};
    }
}


// ===== Camera

pub struct GCamera {

    pub eye: GVector,

    pub ref_up: GVector,

    pub up: GVector,

    pub front: GVector,

    pub right: GVector,

    pub fov: f64,

    pub fov_scale: f64,
}

impl GCamera {

    pub fn new(eye_: GVector, front_: GVector, ref_up_: GVector, fov_: f64) -> GCamera {

        let right_ = GVector::cross_dot(front_, ref_up_);
        let up_ = GVector::cross_dot(right_, front_);

        GCamera {
            eye: eye_,

            front: front_,

            ref_up: ref_up_,

            fov: fov_,

            right: right_,

            up: up_,

            fov_scale: (fov_ * 0.5 * (f64::consts::PI) / 180.0).tan() * 2.0 
        }

    }

    pub fn generate_ray(&self, px: f64, py: f64) -> GRay {

        let r = self.right * ((px - 0.5) * self.fov_scale);
        let u = self.up * ((py - 0.5) * self.fov_scale);
        GRay{ origin: self.eye.clone(), direct: (self.front + r + u).normalize().clone()}

    }

}
