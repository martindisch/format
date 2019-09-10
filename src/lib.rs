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
    // Clone the input so we can manipulate it
    let mut input = s.to_string();
    // Remove any whitespace
    input.retain(|c| !c.is_whitespace());

    let bytes = if input.find("[").is_some() {
        // This appears to be formatted as a byte array, so remove the opening
        // and closing brackets
        let input: String = input.drain(1..input.len() - 1).collect();
        // Split at commas and try to parse values as integers
        input
            .split(",")
            .map(|n| n.parse::<u8>())
            .map(|v| v.map_err(|e| e.into()))
            .collect::<Result<Vec<u8>>>()?
    } else {
        // We have normal hex input
        hex::decode(&input)?
    };

    // Attempt conversion
    Ok(bytes)
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
