//! Materialise the project-authored RGB oracle and codestream for native tests.
use std::path::PathBuf;

fn main() {
    let destination = PathBuf::from(std::env::args_os().nth(1).expect("fixture directory"));
    std::fs::create_dir_all(&destination).unwrap();
    let fixture = emuella_j2k_test_support::native_planes::reversible_mct_region_fixture();
    std::fs::write(destination.join("rgb-mct-256x192.j2k"), fixture.tnsot_one).unwrap();
    let mut rgb = Vec::with_capacity((fixture.width * fixture.height * 3) as usize);
    for index in 0..fixture.planes[0].len() {
        for plane in &fixture.planes {
            rgb.push(plane[index]);
        }
    }
    std::fs::write(destination.join("rgb-mct-256x192.rgb"), rgb).unwrap();
}
