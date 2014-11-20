pub trait ResultUtils<O, E> {
  fn flat_map<A>(self, f: |o: O| -> Result<A, E>) -> Result<A, E>;
}

impl<O, E> ResultUtils<O, E> for Result<O, E> {
  fn flat_map<A>(self, f: |o: O| -> Result<A, E>) -> Result<A, E> {
    match self {
      Ok(success) => f(success),
      Err(fail) => Err(fail)
    }
  }
}

