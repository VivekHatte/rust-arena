use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "converter")]
#[command(version = "1.0")]
#[command(about = "Converts from kilometers to miles and vice versa. Also does the same with farenheit and celcius", long_about = None)]
struct Args {

    #[arg(short, long, value_enum)]
    unit: Unit,

    #[arg(short, long)]
    value: f32,

}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum Unit {
    Kilometers,
    Miles,
    Celsius,
    Fahrenheit,
}
fn main() {

    let args = Args::parse();


    println!("{}", args.value);

    

}

