use geos::{CoordDimensions, CoordSeq, Ordinate};

fn main() {
    let mut coords = CoordSeq::new(1, CoordDimensions::ThreeD).expect("new");
    coords.set_ordinate(0, Ordinate::Z, 10.).expect("ordinate");
    // assert_eq!(coords.get_z(0), Ok(10.));
    assert_eq!(coords.get_ordinate(0, Ordinate::Z), 10.);
}
