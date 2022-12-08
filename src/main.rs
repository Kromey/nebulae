use std::time::Instant;



use nebula::Nebula;

const IMG_DIMENSIONS: usize = 1024;

fn main() {
    let start = Instant::now();

    let dimensions = IMG_DIMENSIONS as u32;

    let nebula = Nebula::new(IMG_DIMENSIONS);
    let pixels: Vec<_> = nebula.generate();

    let img: image::ImageBuffer<image::Rgba<u16>, Vec<u16>> =
        image::ImageBuffer::from_vec(dimensions, dimensions, pixels).unwrap();

    println!(
        "Generated nebula in {:.4} seconds",
        start.elapsed().as_secs_f32()
    );

    // let bytes = Instant::now();
    // let mut buffer = Cursor::new(Vec::with_capacity((dimensions * dimensions) as usize));
    // img.write_to(&mut buffer, image::ImageOutputFormat::Farbfeld).unwrap();
    // let buf = buffer.into_inner();
    // println!(
    //     "Wrote {} bytes to buffer in {:.4} seconds",
    //     buf.len(),
    //     bytes.elapsed().as_secs_f32(),
    // );

    let save = Instant::now();
    img.save("nebula.png").unwrap();
    println!(
        "Wrote image to file in {:.4} seconds",
        save.elapsed().as_secs_f32()
    );
}
