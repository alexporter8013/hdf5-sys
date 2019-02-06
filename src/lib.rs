
mod bindings;
pub use bindings::*; 

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn version() {
        let (mut majnum, mut minnum, mut relnum) = (0, 0, 0);
        assert!(unsafe { H5get_libversion(&mut majnum, &mut minnum, &mut relnum) } >= 0);
        assert_eq!((majnum, minnum, relnum), (1, 10, 4));
    }
}
