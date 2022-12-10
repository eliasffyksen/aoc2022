use std::fmt::Debug;


pub struct AccMap<I, A, F> {
  iter: I,
  acc: Option<A>,
  f: F,
}

impl<B, I: Iterator, A, F> Iterator for AccMap<I, A, F>
where
  A: Debug,
  F: FnMut(A, I::Item) -> (A, B)
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
      let acc = self.acc.take().unwrap();
      let (acc, x) = (self.f)(acc, self.iter.next()?);
      self.acc = Some(acc);
      Some(x)
    }
}

pub trait AccMapExtender<I: Iterator> {
  fn acc_map<A,F,B>(self, init: A, f: F) -> AccMap<I,A,F>
  where
    F: FnMut(A, I::Item) -> (A, B);
}

impl<I> AccMapExtender<I> for I
where
  I: Iterator
{
  fn acc_map<A,F,B>(self, init: A, f: F) -> AccMap<I,A,F>
  where
    F: FnMut(A, I::Item) -> (A, B)
  {
    AccMap {
      iter: self,
      acc: Some(init),
      f,
    }
  }
}
