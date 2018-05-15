pub mod core;
pub mod geom;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
