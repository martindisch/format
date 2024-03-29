use clap::{App, Arg};

use format::Result;

fn main() -> Result<()> {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(
            "A small tool for formatting bytes in various input formats as \
             arrays or CoAP\n\
             messages.",
        )
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("coap")
                .short("c")
                .long("coap")
                .help("Interprets the input as a CoAP message"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("The input bytes as hexstring or byte array")
                .required(true),
        )
        .get_matches();

    let coap = matches.is_present("coap");
    let input = matches.value_of("INPUT").unwrap();

    format::run(input, coap)?;

    Ok(())
}
