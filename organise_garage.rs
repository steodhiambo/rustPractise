use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Garage<T>
where
    T: Add<Output = T> + Copy + Debug,
{
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T> Garage<T>
where
    T: Add<Output = T> + Copy + Debug,
{
    pub fn move_to_right(&mut self) {
        if let (Some(l), r) = (self.left.take(), self.right) {
            self.right = Some(r.map_or(l, |val| val + l));
        }
    }

    pub fn move_to_left(&mut self) {
        if let (l, Some(r)) = (self.left, self.right.take()) {
            self.left = Some(l.map_or(r, |val| val + r));
        }
    }
}
fn main() {
    let mut garage = Garage {
        left: Some(5),
        right: Some(2),
    };

    println!("{:?}", garage);
    garage.move_to_right();
    println!("{:?}", garage);
    garage.move_to_left();
    println!("{:?}", garage);
}
