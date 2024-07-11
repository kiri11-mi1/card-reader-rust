use std::{thread, time::Duration};
use uem_reader::{
    reader::{UemReaderInternalTrait, usb::find_usb_readers},
    commands::{
        UemCommandsTrait,
        reader::UemCommandsReaderTrait,
        cards::{
            UemCommandsCardsTrait,
            UemActivateParameters,
            mifare::{
                UemCommandsCardsMifareTrait,
                classic::UemCommandsCardsMifareClassicTrait
            },
        },
    },
};

fn main() {
    let mut uem_readers = find_usb_readers();

    // Quit if no readers found
    if uem_readers.is_empty() {
        println!("readers not found");
        return;
    }

    // Pick the first reader in the vector
    let uem_reader = uem_readers.get_mut(0);

    // Check if the vector returned an option with valid reader object
    if uem_reader.is_none() {
        println!("uem_reader is none");
        return;
    }

    // Unwrap the option
    let uem_reader = uem_reader.unwrap();

    // Open USB interface connection
    if uem_reader.open().is_err() {
        println!("failed to open USB interface");
        return;
    }

    println!("connected to reader");
    println!("listening for cards...");

    loop {
        // Активирование карты
        let card = uem_reader.commands().cards().activate_a(&UemActivateParameters {
            // switch_to_tcl: true,
            ..Default::default()
        });

        if card.is_err() {
            println!("No card detected. Retrying in 1 second...");
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        // Сигналим если карта обнаружена
        if uem_reader.send(&vec![0x05_u8, 0x01_u8]).is_err() {
            return;
        }

        let card = card.unwrap();
        println!(
            "UUID: {:02X}{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}\n",
            card.uid[0], card.uid[1], card.uid[2], card.uid[3], card.uid[4], card.uid[5], card.uid[6]
        );

        // Аутентификация карты
        // let res = uem_reader.commands().cards().mifare().classic()
        //     .authenticate_key_a(&card, &[0xFF; 6], 1);
        //
        // if res.is_err() {
        //     println!("Failed to authenticate card. Retrying in 1 second...");
        //     thread::sleep(Duration::from_secs(1));
        //     continue;
        // }

        // Чтение данных с карты
        // let res = uem_reader.commands().cards().mifare().classic()
        //     .read(1, 1);
        //
        // if res.is_err() {
        //     println!("Failed to read from card. Retrying in 1 second...");
        //     thread::sleep(Duration::from_secs(1));
        //     continue;
        // }
        //
        // // Вывод данных карты
        // let card_data = res.unwrap();
        // println!("Card data: {:?}", card_data);

        // Ожидание перед следующей итерацией для предотвращения мгновенного повторного чтения
        thread::sleep(Duration::from_secs(1));
    }

    // Закрытие ридера (фактически никогда не выполнится из-за бесконечного цикла)
    if uem_reader.close().is_err() {
        println!("Failed to close reader.");
    }
}