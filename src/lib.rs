
#[macro_use]
pub mod util {
    macro_rules! expect_variant {
        ($value: expr, $variant: pat => $then: expr) => {
            if let $variant = $value {
                $then

            } else {
                panic!("Expected variant {} in {}", stringify!($variant), stringify!($value))
            }
        };
    }
}


pub mod file;
pub mod image;

extern crate seek_bufread;
extern crate libflate;
extern crate bit_field;
extern crate byteorder;
extern crate half;

#[macro_use]
extern crate smallvec;

#[cfg(test)]
extern crate image as piston_image;


// TODO various compiler tweaks, such as export RUSTFLAGS='-Ctarget-cpu=native'

pub mod prelude {
//    pub use file::io::read_file;
    pub use file::io::ReadError;

//    pub use file::io::write_file;
    pub use file::io::WriteError;

    pub use file::meta::MetaData;
}



#[cfg(test)]
pub mod test {

//    #[bench]
//    fn load_meta_only(){
//      TODO
//    }


    // TODO erroneous files:
    // "Blobbies.exr"
    // "composited.exr"
    // "Fog.exr"
    // "WavyLinesSphere.exr"
    // "WideColorGamut.exr"
    // "RgbRampsDiagonal.exr"
    // "GammaChart.exr"
    // "GrayRampsDiagonal.exr"
    // "BrightRingsNanInf.exr"
    // "AllHalfValues.exr"
    // "BrightRings.exr"
    // "GrayRampsHorizontal.exr"
    // "singlepart.0007.exr"
    // "singlepart.0001.exr"
    // "singlepart.0008.exr"
    // "singlepart.0002.exr"
    // "singlepart.0004.exr"
    // "singlepart.0006.exr"
    // "singlepart.0003.exr"
    // "singlepart.0005.exr"
    // "suzanne_rgba_f32_rle.exr"
    // "suzanne_rgba_f32_uncompressed.exr"
    // "suzanne_rgba_f16_uncompressed.exr"
    // "suzanne_rgba_f32_zips.exr"
    // "singlepart.0003.exr"

    use ::std::path::Path;

    #[test]
    fn read_all_files() {

        // TODO test if reading pushed the reader to the very end of the file?

        fn test_exr_files(path: &Path){
            if let Some("exr") = path.extension().and_then(|os| os.to_str()) {
                print!("testing file {:?}... ", path.file_name().unwrap());
                load_file_or_print_err(path)

            } else if path.is_dir() {
                for sub_dir in ::std::fs::read_dir(path).unwrap() {
                    test_exr_files(&sub_dir.unwrap().path());
                }
            }
        }

        test_exr_files(Path::new("/home/johannes/Pictures/openexr"))
    }

    #[test]
    fn read_file(){
        load_file_or_print_err(Path::new("/home/johannes/Pictures/openexr/openexr-images-master/ScanLines/Blobbies.exr"))
    }

    fn load_file_or_print_err(path: &Path){
        println!(
            "{:?}",
            ::image::immediate::read_raw_parts(
                &mut ::std::fs::File::open(path).unwrap()
            ).map(|_| "no errors")
        );

        //println!("{}", ::image::immediate::read_file(path).map(|_| "no errors").unwrap());
    }


    #[test]
    fn convert_to_png() {
        use std::time::Instant;

        let now = Instant::now();

        let path = ::std::path::Path::new(
            "/home/johannes/Pictures/openexr/suzanne/suzanne_rgba_f32_uncompressed.exr"
        );

        let image = ::image::immediate::read_file(path);

        // warning: highly unscientific benchmarks ahead!
        let elapsed = now.elapsed();
        let millis = elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64;

        if let Ok(image) = image {
            println!("header_0: {:#?}", image.parts[0].header);
            println!("\nversion: {:#?}", image.version);
            println!("\ndecoded file in {:?} ms", millis);

            let header = &image.parts[0].header;
            let channels = &image.parts[0].levels[0];
            let full_res = header.data_window().dimensions();

            let mut png_buffer = ::piston_image::GrayImage::new(full_res.0, full_res.1);

            // BUGHUNT CHECKLIST
            // - [ ] rust-f16 encoding is not the same as openexr-f16 encoding
            // - [ ] compression differs from specification


            // convert to png
            expect_variant!(channels, ::image::immediate::PartData::Flat(ref channels) => {
                expect_variant!(channels[2], ::file::data::uncompressed::Array::F32(ref channel) => {
                    for (x, y, pixel) in png_buffer.enumerate_pixels_mut() {
                        let v = channel[(y * full_res.0 + x) as usize]/*.to_f32()*/;
                        *pixel = ::piston_image::Luma([(v * 255.0) as u8]);
                    }
                })
            });

            png_buffer.save(path.with_extension("png").file_name().unwrap()).unwrap();


        } else {
            println!("Error: {:?}", image.err().unwrap());
        }
    }

    // TODO allow loading only meta data,
    // TODO and allow seek-loading tiles based on offset tables afterwards

    // TODO check for completeness of file
    // TODO handle incomplete files based on if the offset_table is complete (last thing written)
    // TODO memory-mapping

    // TODO let the user decide how to store something,
    // don't just read the pixels into a buffer and let the user convert the data into new data again
    // in order to avoid too much memory allocations
    // (something like  read_pixels(|index, pixel| pixels[index] = RGBA::new(pixel[0], pixel[1], ...) )
}
