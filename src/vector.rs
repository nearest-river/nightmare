
use core::ops::*;


pub trait VectorExt {
  type T: Add+Sub+Mul+Div+Rem+AddAssign+SubAssign+MulAssign+DivAssign;
  fn norm(self)-> Self::T;
  fn add(self,other: Self)-> Vec<Self::T>;
  fn sub(self,other: Self)-> Vec<Self::T>;
  fn mul(self,other: Self)-> Vec<Self::T>;
  fn div(self,other: Self)-> Vec<Self::T>;

  fn dot(self,other: Self)-> Self::T;
}


impl VectorExt for &[f32] {
  type T=f32;
  
  fn norm(self)-> f32 {
    self.iter()
    .map(|x| x*x)
    .sum::<f32>()
    .sqrt()
  }

  fn add(self,other: Self)-> Vec<f32> {
    self.iter()
    .zip(other)
    .map(|(a,b)| a+b)
    .collect::<Vec<_>>()
  }

  fn sub(self,other: Self)-> Vec<f32> {
    self.iter()
    .zip(other)
    .map(|(a,b)| a-b)
    .collect::<Vec<_>>()
  }

  fn mul(self,other: Self)-> Vec<f32> {
    self.iter()
    .zip(other)
    .map(|(a,b)| a*b)
    .collect::<Vec<_>>()
  }

  fn div(self,other: Self)-> Vec<f32> {
    self.iter()
    .zip(other)
    .map(|(a,b)| a/b)
    .collect::<Vec<_>>()
  }

  fn dot(self,other: Self)-> f32 {
    self.iter()
    .zip(other)
    .map(|(a,b)| a*b)
    .sum::<f32>()
    .sqrt()
  }
}









