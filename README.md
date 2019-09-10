# format
A small tool for formatting bytes in various input formats as arrays or CoAP
messages.

## Usage
Enter your bytes either as an array of numbers or as a hexstring and see them
formatted as an array of hexadecimals, or when using `-c/--coap` as the debug
representation of a CoAP message.

Format an array of decimals as an array of hexadecimals
```console
$ format "[72, 101, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]"
[0x48, 0x65, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21]
```

Format a hexstring as a CoAP message
```console
$ format -c 44015d1f00003974396c6f63616c686f737483747631
Packet {
    header: Header {
        ver_type_tkl: 100,
        code: Response(
            Content,
        ),
        message_id: 23839,
    },
    token: [
        0,
        0,
        57,
        116,
    ],
    options: {},
    payload: [
        72,
        101,
        108,
        108,
        111,
        32,
        87,
        111,
        114,
        108,
        100,
        33,
    ],
}
```

## License
Licensed under either of

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
