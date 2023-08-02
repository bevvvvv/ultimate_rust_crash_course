// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let matches = App::new("Image Processing")
                        .version("0.1")
                        .author("Joseph S. <joe@sepich.dev>")
                        .about("This application performs various image processing tasks")
                        .subcommand(SubCommand::with_name("blur")
                            .about("Blur an image")
                            .arg(Arg::with_name("input")
                                .short("i")
                                .help("Input file")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("output")
                                .short("o")
                                .help("Output file")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("blur_amount")
                                .long("blur")
                                .help("Blur amount")
                                .takes_value(true)
                                .required(true)
                                .default_value("2.0")))
                        .subcommand(SubCommand::with_name("brighten")
                            .about("Change the brightness of an image")
                            .arg(Arg::with_name("input")
                                .short("i")
                                .help("Input file")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("output")
                                .short("o")
                                .help("Output file")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("brightness_amount")
                                .long("level")
                                .help("Brightness Level Integer")
                                .takes_value(true)
                                .required(true)
                                .default_value("2")))
                        .subcommand(SubCommand::with_name("crop")
                            .about("Crop an image")
                            .arg(Arg::with_name("input")
                                .short("i")
                                .help("Input file")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("output")
                                .short("o")
                                .help("Output file")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("x_coord")
                                .short("x")
                                .help("Starting X Coordinate")
                                .takes_value(true))
                            .arg(Arg::with_name("y_coord")
                                .short("y")
                                .help("Starting Y Coordinate")
                                .takes_value(true))
                            .arg(Arg::with_name("width")
                                .short("w")
                                .help("Width of cropped image")
                                .takes_value(true))
                            .arg(Arg::with_name("height")
                                .short("h")
                                .help("Height of cropped image")
                                .takes_value(true)))
                        .subcommand(SubCommand::with_name("rotate")
                            .about("Rotate an image in 90 degree increments")
                            .arg(Arg::with_name("input")
                                .short("i")
                                .help("Input file")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("output")
                                .short("o")
                                .help("Output file")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("angle")
                                .long("angle")
                                .help("Rotation Angle in degrees interval of 90")
                                .takes_value(true)))
                        .subcommand(SubCommand::with_name("invert")
                            .about("Invert the colors of an image")
                            .arg(Arg::with_name("input")
                                .short("i")
                                .help("Input file")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("output")
                                .short("o")
                                .help("Output file")
                                .takes_value(true)
                                .required(true)))
                        .subcommand(SubCommand::with_name("grayscale")
                            .about("Convert image to grayscale")
                            .arg(Arg::with_name("input")
                                .short("i")
                                .help("Input file")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("output")
                                .short("o")
                                .help("Output file")
                                .takes_value(true)
                                .required(true)))
                        .subcommand(SubCommand::with_name("generate")
                            .about("Generate an image")
                            .arg(Arg::with_name("output")
                                .short("o")
                                .help("Output file")
                                .takes_value(true)
                                .required(true)))
                        .subcommand(SubCommand::with_name("fractal")
                            .about("Generate a fractal")
                            .arg(Arg::with_name("output")
                                .short("o")
                                .help("Output file")
                                .takes_value(true)
                                .required(true))).get_matches();

    match matches.subcommand() {
        ("blur", Some(blur_command)) => {
            let input = blur_command.value_of("input").unwrap().to_string();
            let output = blur_command.value_of("output").unwrap().to_string();
            let blur_amount = blur_command.value_of("blur_amount").unwrap().parse().expect("Failed to parse blur amount");
            blur(input, output, blur_amount);
        },
        ("brighten", Some(brighten_command)) => {
            let input = brighten_command.value_of("input").unwrap().to_string();
            let output = brighten_command.value_of("output").unwrap().to_string();
            let brightness_amount = brighten_command.value_of("brightness_amount").unwrap().parse().expect("Failed to parse brightness level");
            brighten(input, output, brightness_amount);
        },
        ("crop", Some(crop_command)) => {
            let input = crop_command.value_of("input").unwrap().to_string();
            let output = crop_command.value_of("output").unwrap().to_string();
            let x = crop_command.value_of("x_coord").unwrap().parse().expect("Failed to parse x coordinate");
            let y = crop_command.value_of("y_coord").unwrap().parse().expect("Failed to parse y coordinate");
            let width = crop_command.value_of("width").unwrap().parse().expect("Failed to parse width");
            let height = crop_command.value_of("height").unwrap().parse().expect("Failed to parse height");
            crop(input, output, x, y, width, height);
        },
        ("rotate", Some(rotate_command)) => {
            let input = rotate_command.value_of("input").unwrap().to_string();
            let output = rotate_command.value_of("output").unwrap().to_string();
            let angle = rotate_command.value_of("angle").unwrap().parse().expect("Failed to parse angle");
            rotate(input, output, angle);
        },
        ("invert", Some(invert_command)) => {
            let input = invert_command.value_of("input").unwrap().to_string();
            let output = invert_command.value_of("output").unwrap().to_string();
            invert(input, output);
        },
        ("grayscale", Some(grayscale_command)) => {
            let input = grayscale_command.value_of("input").unwrap().to_string();
            let output = grayscale_command.value_of("output").unwrap().to_string();
            grayscale(input, output);
        },
        ("generate", Some(generate_command)) => {
            let output = generate_command.value_of("output").unwrap().to_string();
            generate(output);
        },
        ("fractal", Some(fractal_command)) => {
            let output = fractal_command.value_of("output").unwrap().to_string();
            fractal(output);
        }
        _ => {
            println!("Could not parse command.");
        },
    }
}

fn blur(infile: String, outfile: String, blur_amount: f32) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open input file.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(blur_amount);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing to output file.");
}

fn brighten(infile: String, outfile: String, brightness_amount: i32) {
    // See blur() for an example of how to open / save an image.
    let img = image::open(infile).expect("Failed to open input file.");

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img2 = img.brighten(brightness_amount);

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
    img2.save(outfile).expect("Failed writing to output file.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open input file.");

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.
    let img2 = img.crop_imm(x, y, width, height);

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing to output file.");
}

fn rotate(infile: String, outfile: String, angle: u32) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open input file.");

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!
    let mut img2 = img.clone();
    match angle {
        90 => img2 = img2.rotate90(),
        180 => img2 = img2.rotate180(),
        270 => img2 = img2.rotate270(),
        _ => img2 = img2.rotate90(),
    }

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing to output file.");
}

fn invert(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let mut img = image::open(infile).expect("Failed to open input file.");

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    img.invert();

    // See blur() for an example of how to save the image.
    img.save(outfile).expect("Failed writing to output file.");
}

fn grayscale(infile: String, outfile: String) {
    // See blur() for an example of how to open an image.
    let img = image::open(infile).expect("Failed to open input file.");

    // .grayscale() takes no arguments. It returns a new image.
    let img2 = img.grayscale();

    // See blur() for an example of how to save the image.
    img2.save(outfile).expect("Failed writing to output file.");
}

fn generate(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example
    let mut imgbuffer = image::ImageBuffer::new(800, 800);

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    // Set the image to some solid color. -- see fractal() for an example
    for (x, y, pixel) in imgbuffer.enumerate_pixels_mut() {
        let red: u8 = 234;
        let blue: u8 = 150;
        let green: u8 = 0;

        *pixel = image::Rgb([red, green, blue]);
    }

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
    imgbuffer.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
