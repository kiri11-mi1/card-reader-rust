use uem_reader::{
    reader::{
        UemReaderInternalTrait,
        usb::find_usb_readers
    },
};


fn main() {
    // Search system for USB readers
    let mut uem_readers = find_usb_readers();

    // Quit if no readers found
    if uem_readers.is_empty() {
        return;
    }

    // Pick the first reader in the vector
    let uem_reader = uem_readers.get_mut(0);

    // Check if the vector returned an option with valid reader object
    if uem_reader.is_none() {
        return;
    }

    // Unwrap the option
    let uem_reader = uem_reader.unwrap();

    // Open USB interface connection
    if uem_reader.open().is_err() {
        return;
    }

    // Beep 1 time using command byte vector
    if uem_reader.send(&vec![0x05_u8, 0x01_u8]).is_err() {
        return;
    }
}