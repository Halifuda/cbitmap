#[cfg(test)]
mod should_panic {
    extern crate cbitmap;
    use cbitmap::bitmap::*;

    #[test]
    #[should_panic]
    fn at_out_of_range() {
        let _ = Bitmap::<1>::new().at(8);
    }

    #[test]
    #[should_panic]
    fn at_mut_out_of_range() {
        let _ = (&mut Bitmap::<1>::new()).at_mut(8);
    }

    #[test]
    #[should_panic]
    fn set_mut_out_of_range() {
        (&mut Bitmap::<1>::new()).set(8);
    }

    #[test]
    #[should_panic]
    fn reset_mut_out_of_range() {
        (&mut Bitmap::<1>::new()).reset(8);
    }

    #[test]
    #[should_panic]
    fn flip_mut_out_of_range() {
        let _ = (&mut Bitmap::<1>::new()).flip(8);
    }
}