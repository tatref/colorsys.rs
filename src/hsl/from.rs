use super::Hsl;
use crate::{ColorTuple, ColorTupleA};

impl std::convert::From<&ColorTuple> for Hsl {
  fn from(t: &ColorTuple) -> Hsl {
    let (h, s, l) = *t;
    Hsl::new(h, s, l, None)
  }
}

impl std::convert::From<ColorTuple> for Hsl {
  fn from(t: ColorTuple) -> Hsl {
    Hsl::from(&t)
  }
}

impl From<&ColorTupleA> for Hsl {
  fn from(t: &ColorTupleA) -> Hsl {
    let (h, s, l, a) = *t;
    Hsl::new(h, s, l, Some(a))
  }
}

impl From<ColorTupleA> for Hsl {
  fn from(t: ColorTupleA) -> Hsl {
    Hsl::from(&t)
  }
}

// impl From<&[f32]> for Hsl {
//   fn from(a: &[f32]) -> Hsl {
//     let h = a.get(0).cloned().unwrap_or(0.0);
//     let s = a.get(1).cloned().unwrap_or(0.0);
//     let l = a.get(2).cloned().unwrap_or(0.0);
//     let a = a.get(3).cloned();
//     Hsl::new(h, s, l, a)
//   }
// }

//
//
//
// INTO
//
impl<'a> Into<ColorTuple> for &'a mut Hsl {
  fn into(self) -> ColorTuple {
    let Hsl { h, s, l, .. } = *self;
    (h, s, l)
  }
}
impl<'a> Into<ColorTuple> for &'a Hsl {
  fn into(self) -> ColorTuple {
    let Hsl { h, s, l, .. } = *self;
    (h, s, l)
  }
}

impl Into<ColorTuple> for Hsl {
  fn into(self) -> ColorTuple {
    self.as_tuple()
  }
}

impl<'a> Into<ColorTupleA> for &'a Hsl {
  fn into(self) -> ColorTupleA {
    let Hsl { h, s, l, .. } = *self;
    (h, s, l, self.get_alpha())
  }
}
impl<'a> Into<ColorTupleA> for &'a mut Hsl {
  fn into(self) -> ColorTupleA {
    let Hsl { h, s, l, .. } = *self;
    (h, s, l, self.get_alpha())
  }
}

impl Into<ColorTupleA> for Hsl {
  fn into(self) -> ColorTupleA {
    self.as_tuple_with_alpha()
  }
}