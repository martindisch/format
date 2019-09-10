use coap_lite::Packet;
use std::error::Error;

/// The generic result type for this crate.
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Prints the input formatted as a hexadecimal byte array or CoAP message.
pub fn run(input: &str, coap: bool) -> Result<()> {
    // Interpret the input
    let bytes = to_bytes(input)?;
    // Format as requested
    let output = if coap {
        format_coap(&bytes)?
    } else {
        format_hexarray(&bytes)
    };

    println!("{}", output);

    Ok(())
}

/// Converts the input (hexstring or byte array notation) to bytes.
fn to_bytes(s: &str) -> Result<Vec<u8>> {
    // TODO: Implement byte array notation
    // Clone the input so we can manipulate it
    let mut input = s.to_string();
    // Remove any whitespace
    input.retain(|c| !c.is_whitespace());

    // Attempt conversion
    Ok(hex::decode(&input)?)
}

/// Returns the bytes formatted as a byte array in hexadecimal notation with
/// capital letters.
fn format_hexarray(bytes: &[u8]) -> String {
    String::from("[0x")
        + &bytes
            .iter()
            .map(|n| format!("{:02X}", n))
            .collect::<Vec<String>>()
            .join(", 0x")
        + "]"
}

/// Returns the bytes formatted as a CoAP message.
fn format_coap(slice: &[u8]) -> Result<String> {
    // Attempt parsing as CoAP packet
    let packet = Packet::from_bytes(slice)?;
    // Return debug formatting
    Ok(format!("{:#?}", packet))
}
