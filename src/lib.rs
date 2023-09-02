// =============================================================================
//! - CroftSoft Roles Library
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-18
//! - Updated: 2023-09-02
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub trait Initializer {
  fn initialize(&self);
}

pub trait InitializerMut {
  fn initialize(&mut self);
}

pub trait Painter {
  fn paint(&self);
}

pub trait PainterMut {
  fn paint(&mut self);
}

pub trait Preparer {
  fn prepare(&self);
}

pub trait PreparerMut {
  fn prepare(&mut self);
}

pub trait Updater {
  fn update(&self);
}

pub trait UpdaterMut {
  fn update(&mut self);
}
