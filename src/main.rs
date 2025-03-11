use clap::Parser;
use curl_parser::converters::nodejs::Axios;
use curl_parser::converters::python::Requests;
use curl_parser::converters::Convert;
use curl_parser::{parse_curl_command, ConversionFormat, ParseError};
use std::process;

#[derive(Parser)]
struct Cli {
    
    #[arg(required = true)]
    curl_command: String,

    
    #[arg(short, long, default_value = "python-requests")]
    format: String,
}

fn main() {
    let args = Cli::parse();

    
    let result = parse_curl_command(&args.curl_command);

    match result {
        Ok(parsed) => {
            
            match args.format.parse::<ConversionFormat>() {
                Ok(format) => match format {
                    ConversionFormat::PythonRequests => {
                        let converter = Requests;
                        println!("{}", converter.convert(&parsed));
                    }
                    ConversionFormat::NodeJS => {
                        let converter = Axios;
                        println!("{}", converter.convert(&parsed));
                    }
                },
                Err(err) => {
                    eprintln!("Error: {}", err);
                    process::exit(1);
                }
            }
        }
        Err(err) => {
            match err {
                ParseError::MissingUrl => {
                    eprintln!("Error: Missing URL in curl command");
                }
                ParseError::InvalidUrl(err) => {
                    eprintln!("Error: Invalid URL: {}", err);
                }
                ParseError::ParseFailure(msg) => {
                    eprintln!("Error: {}", msg);
                }
            }
            process::exit(1);
        }
    }
}
