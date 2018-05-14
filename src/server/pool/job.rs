pub trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    println!("I is here");
    //(*self)()
  }
}

pub type Job = Box<FnBox + Send + 'static>;
