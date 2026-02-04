pub trait Capture<T> {
  fn capture(source: &T) -> Self;
}
