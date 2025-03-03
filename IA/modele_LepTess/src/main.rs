fn main() {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image("../image/test4.jpg");
    println!("{}", lt.get_utf8_text().unwrap());
}
