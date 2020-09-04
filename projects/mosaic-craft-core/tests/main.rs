use mosaic_craft::repack_directory;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let pack = repack_directory("../mosaic-craft-themes/minecraft3d", "../mosaic-craft-themes/minecraft3d.mosaic-craft-theme")
        .unwrap();
    println!("{:?}", pack)
}
