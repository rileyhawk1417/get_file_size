use std::env;
use std::error::Error;
use std::fs;
use std::process;
use bytesize::ByteSize;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Invalid use: {}", err);
        process::exit(1);
    });

    //TODO: Avoid 'src' name so more like define a range
    /* 
    let file_namez: Vec<&str> = Regex::new(r"\w+")
        .unwrap()
        .find_iter(&args[1])
        .map(|x| x.as_str())
        .collect();
    */

    
    if let Err(e) = compare_size(config){
        println!("App Error: {}", e);
        process::exit(1);
    }
}

struct Config {
    dest_file: String,
    src_file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Usage: ./get_file_size 'file_a' 'file_b' ");
        }
        let src_file: String = args[1].clone();
        let dest_file: String = args[2].clone();

        Ok(Config {
            src_file,
            dest_file,
        })
    }
}

fn compare_size(config: Config) -> Result<(), Box<dyn Error>>{
    let file_a : fs::Metadata= fs::metadata(config.src_file.as_str())?;
    let file_b:fs::Metadata = fs::metadata(config.dest_file.as_str())?;

    if file_a.len() < file_b.len() {
        println!("Name: {}, Size: {}", &config.src_file, bytes_to_mb(file_a.len()));
        println!("Name: {}, Size: {}", &config.dest_file, bytes_to_mb(file_b.len()));
        println!("{} is smaller than {}", &config.src_file, &config.dest_file);
    } else if file_a.len() > file_b.len() {
        println!("Name: {}, Size: {}", &config.src_file, bytes_to_mb(file_a.len()));
        println!("Name: {}, Size: {}", &config.dest_file, bytes_to_mb(file_b.len()));
        println!("{} is bigger than {}", &config.dest_file, &config.src_file);
    } else {
        println!("Cannot compare {} & {}", &config.src_file, &config.dest_file);
    }
    Ok(())
}
fn bytes_to_mb(size: u64)  -> ByteSize {
    let final_result: ByteSize = ByteSize::b(size);
    return final_result;
}