use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};



pub fn print_green(text: &str){
    // Create a color specification for green
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Green));

    //Create a standard stream for writing to the console
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Write the text to the console with the green color
    stdout.set_color(&spec).unwrap();
    writeln!(&mut stdout, "{}", text).unwrap();

    //Reset the color specification to the default
    stdout.reset().unwrap();

}