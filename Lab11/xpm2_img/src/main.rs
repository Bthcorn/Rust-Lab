fn main() {
    let checker = "\
        ! XPM2\n\
        4 4 2 1\n\
        # c #000000\n\
        - c #FFFFFF\n\
        ##--\n\
        ##--\n\
        --##\n\
        --##\n\
    ";
    // let data = checker.as_bytes().to_vec();
    let data = checker.as_bytes().to_vec();
    let mut reader: &[u8] = data.as_slice();
    let img = xpm2_img::read_xpm2(&mut reader).unwrap();


    let _ = xpm2_img::write_to_svg(&img, "output.svg");
    // ---------------------------------
    let ctable = &[
        ("#".into(), "#000000".into()),  // c1
        ("-".into(), "#808080".into()),  // c2
    ];
    let rows = ["##--", "----", "####"];
    let pixels: Vec<_> = rows.iter().map(|r| r.to_string()).collect();
    let img = xpm2_img::make_xpm2(ctable, &pixels);

    let pixel_size = 20;

    let _ = xpm2_img::write_to_svg_pixel(&img, pixel_size, "output1.svg");

}
