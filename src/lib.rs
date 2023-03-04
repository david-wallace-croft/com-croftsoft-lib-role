// =============================================================================
//! - CroftSoft Roles Library
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-01-18
//! - Updated: 2023-03-03
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub trait Initializer {
  fn initialize(&mut self);
}

pub trait Painter {
  fn paint(&mut self);
}

pub trait Updater {
  fn update(&mut self);
}
