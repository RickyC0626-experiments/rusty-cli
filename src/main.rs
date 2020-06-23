use structopt::StructOpt;

// Search for a pattern in a file and display the lines that contain it
#[derive(StructOpt)]
#[derive(Debug)]
struct CLI
{
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() 
{
    let args = CLI::from_args();
    println!("rusty-cli: {:?} {:?}", args.pattern, args.path);
}