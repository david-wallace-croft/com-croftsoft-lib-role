pub trait Updater<C> {
  fn update(
    &mut self,
    context: &C,
  );
}
