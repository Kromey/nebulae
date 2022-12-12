
fn main() {
    let img = sky::generate(0xCAFEBABE);

    img.save("nebula.png").unwrap();
}
