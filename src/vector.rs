
use core::ops::*;


pub trait VectorExt {
  type T: Add+Sub+Mul+Div+Rem+AddAssign+SubAssign+MulAssign+DivAssign;
  fn norm(self)-> Self::T;
  fn add(self,other: Self)-> Vec<Self::T>;
  fn sub(self,other: Self)-> Vec<Self::T>;
  fn mul(self,other: Self)-> Vec<Self::T>;
  fn div(self,other: Self)-> Vec<Self::T>;

  fn dot(self,other: Self)-> Self::T;
  fn cosine_similarity(self,other: Self)-> Self::T;
}


impl VectorExt for &[f64] {
  type T=f64;
  
  fn norm(self)-> f64 {
    self.iter()
    .map(|x| x*x)
    .sum::<f64>()
    .sqrt()
  }

  fn add(self,other: Self)-> Vec<f64> {
    self.iter()
    .zip(other)
    .map(|(a,b)| a+b)
    .collect::<Vec<_>>()
  }

  fn sub(self,other: Self)-> Vec<f64> {
    self.iter()
    .zip(other)
    .map(|(a,b)| a-b)
    .collect::<Vec<_>>()
  }

  fn mul(self,other: Self)-> Vec<f64> {
    self.iter()
    .zip(other)
    .map(|(a,b)| a*b)
    .collect::<Vec<_>>()
  }

  fn div(self,other: Self)-> Vec<f64> {
    self.iter()
    .zip(other)
    .map(|(a,b)| a/b)
    .collect::<Vec<_>>()
  }

  fn dot(self,other: Self)-> f64 {
    self.iter()
    .zip(other)
    .map(|(a,b)| a*b)
    .sum::<f64>()
    .sqrt()
  }

  #[inline]
  fn cosine_similarity(self,other: Self)-> Self::T {
    self.dot(other)/(self.norm()*other.norm())
  }
}









