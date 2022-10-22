fn main() -> nfc1::Result<()> {
    let mut context = nfc1::Context::new()?;
    let mut device = context.open()?;

    match device.initiator_select_passive_target(&nfc1::Modulation {
        modulation_type: nfc1::ModulationType::Iso14443a,
        baud_rate: nfc1::BaudRate::Baud106,
    }) {
        Ok(target) => print!("Found a {}", target.to_string(false)?),
        Err(_) => print!("Error, or nothing found"),
    }

    Ok(())
}
