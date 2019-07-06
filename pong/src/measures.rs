use std::ops::{
    Div, Mul
};

pub use velocity::Velocity;

mod velocity {
    use std::ops::Mul;
    use super::{Time, Distance};

    #[derive(Copy, Clone)]
    pub struct Velocity(f32);

    impl Velocity {
        pub fn new(value: f32) -> Velocity {
            Velocity(value)
        }
    }

    impl Mul<Time> for Velocity {
        type Output = Distance;

        fn mul(self, rhs: Time) -> Self::Output {
            Distance(self.0 * rhs.0)
        }
    }
}



#[derive(Copy, Clone)]
pub struct Time(pub f32);

#[derive(Copy, Clone)]
pub struct Distance(pub f32);

#[derive(Copy, Clone)]
pub struct Height(pub f32);

#[derive(Copy, Clone)]
pub struct Width(pub f32);

#[derive(Copy, Clone)]
pub struct YCoord(pub f32);

#[derive(Copy, Clone)]
pub struct XCoord(pub f32);

impl Div<Time> for Distance {
    type Output = Velocity;

    fn div(self, rhs: Time) -> Self::Output {
        Velocity::new(self.0 / rhs.0)
    }
}
