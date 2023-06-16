use std::{io::Read, process::Command};

pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Date {
    pub fn new() -> Self {
        let mut string = String::new();
        Command::new("/bin/bash")
            .arg("-c")
            .arg("date +%F")
            .output()
            .unwrap()
            .stdout
            .as_slice()
            .read_to_string(&mut string)
            .unwrap();

        let bytes = string.as_bytes();

        let year = &bytes[0..4];
        let month = &bytes[5..7];
        let day = &bytes[8..10];

        Date {
            day: ((day[0] - 48) * 10) + (day[1] - 48),
            month: ((month[0] - 48) * 10) + (month[1] - 48),
            year: ((year[0] as u16 - 48) * 1000)
                + ((year[1] as u16 - 48) * 100)
                + ((year[2] as u16 - 48) * 10)
                + (year[3] as u16 - 48),
        }
    }
}
