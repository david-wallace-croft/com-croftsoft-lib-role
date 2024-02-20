// =============================================================================
//! - CroftSoft Roles Library
//!
//! # Metadata
//! - Copyright: &copy; 2023-2024 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-18
//! - Updated: 2024-02-19
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub trait Initializer<T = ()> {
  fn initialize(&self) -> T;
}

pub trait InitializerMut<T = ()> {
  fn initialize(&mut self) -> T;
}

pub trait Painter<T = ()> {
  fn paint(&self) -> T;
}

pub trait PainterMut<T = ()> {
  fn paint(&mut self) -> T;
}

pub trait Preparer<T = ()> {
  fn prepare(&self) -> T;
}

pub trait PreparerMut<T = ()> {
  fn prepare(&mut self) -> T;
}

pub trait Updater<T = ()> {
  fn update(&self) -> T;
}

pub trait UpdaterMut<T = ()> {
  fn update(&mut self) -> T;
}

pub trait Validator<T = ()> {
  fn validate(&self) -> T;
}

pub trait ValidatorMut<T = ()> {
  fn validate(&mut self) -> T;
}
