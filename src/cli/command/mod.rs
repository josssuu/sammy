use std::io;

pub mod check;
pub mod update;
pub mod woof;

trait Reportable {
    fn display(&self) -> io::Result<()>;
}