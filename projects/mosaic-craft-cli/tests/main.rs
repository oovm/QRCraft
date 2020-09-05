use mosaic_craft_core::{repack_all_theme, repack_directory, LegoArtError};

#[test]
fn ready() {
    println!("it works!")
}

#[ignore]
#[test]
fn pack_all_theme() {
    match repack_all_theme("../mosaic-craft-themes", "../mosaic-craft-themes") {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
