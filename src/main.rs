const FIRST_CMD: [u8; 1] = [0x60];

fn main() -> nfc1::Result<()> {
    let mut context = nfc1::Context::new()?;
    let mut device = context.open()?;

    match device.initiator_select_passive_target(&nfc1::Modulation {
        modulation_type: nfc1::ModulationType::Iso14443a,
        baud_rate: nfc1::BaudRate::Baud424,
    }) {
        Ok(_target) => {
            match device.initiator_transceive_bytes(&FIRST_CMD, 8, nfc1::Timeout::Default){
                Ok(rx) => {
                    if rx[0] == 0xaf {
                        print!("Use 0xaf");
                    }
                },
                Err(err) => print!("Error {}\n", err),
            };
        },
        Err(_) => print!("Error, or nothing found"),
    }

    Ok(())
}
