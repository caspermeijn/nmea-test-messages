// Copyright (C) 2024 Casper Meijn <casper@meijn.net>
//
// SPDX-License-Identifier: CC0-1.0

struct UdpSocket {
    socket: std::net::UdpSocket
}

impl UdpSocket {
    fn new() -> Self {
        Self {
            socket: std::net::UdpSocket::bind("127.0.0.1:2000").unwrap(),
        }
    }

    fn write_payload(&self, payload: &str) {
        self.socket.send_to(payload.as_bytes(), "127.0.0.1:2000").unwrap();
    }

    fn write_sentence_without_checksum(&self, without_checksum: &str) {
        let checksum = without_checksum.chars().skip(1).fold(0, |sum, c| sum ^ c as u8);
        self.write_payload(&format!("{without_checksum}*{checksum:02x}\r\n"));
    }

    fn write_text_sentence(&self, text: &str) {
        self.write_sentence_without_checksum(&format!("$GPTXT,01,01,{text}"));
    }

}

fn main() {
    let w = UdpSocket::new();
    w.write_text_sentence("Next 1 sentence is a valid message");
    w.write_sentence_without_checksum("$GPGGA,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 3 sentence has invalid newline");
    w.write_payload("$GPGGA,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*42");
    w.write_payload("$GPGGA,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*42\r");
    w.write_payload("$GPGGA,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*42\n");
    w.write_text_sentence("Following 1 sentence has incorrect checksum");
    w.write_payload("$GPGGA,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*99\r\n");
    w.write_text_sentence("Following 4 sentence is missing a checksum");
    w.write_payload("$GPGGA,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*9\r\n");
    w.write_payload("$GPGGA,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*\r\n");
    w.write_payload("$GPGGA,225239.784,2,N,01217.461,E,,5,3.5,,,,,,\r\n");
    w.write_payload("$GPGGA,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 1 sentence starts with invalid character");
    w.write_sentence_without_checksum("#GPGGA,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 1 sentence ha unknown talker id");
    w.write_sentence_without_checksum("$ZZGGA,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 1 sentence has unknown sentence id");
    w.write_sentence_without_checksum("$GPZZZ,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 1 sentence has too long sentence id");
    w.write_sentence_without_checksum("$GPZZZZZZ,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 2 sentence has too short sentence id");
    w.write_sentence_without_checksum("$GPGG");
    w.write_payload("$GPGG");
    w.write_text_sentence("Following 1 sentence has no fields");
    w.write_sentence_without_checksum("$GPGGA,");
    
    w.write_text_sentence("Following 1 sentence is a valid ZDA message");
    w.write_sentence_without_checksum("$GPZDA,225239.91,14,11,2015,+01,00");
    w.write_text_sentence("Following 1 sentence is a valid ZDA message with only empty fields");
    w.write_sentence_without_checksum("$GPZDA,,,,,,");
    w.write_text_sentence("Following 1 sentence is a valid ZDA message without second fraction");
    w.write_sentence_without_checksum("$GPZDA,225239,14,11,2015,+01,00");
    w.write_text_sentence("Following 1 sentence is a valid ZDA message with negative timezone");
    w.write_sentence_without_checksum("$GPZDA,225239,14,11,2015,-07,30");
    w.write_text_sentence("Following 1 sentence is a valid ZDA message with negative zero timezone");
    w.write_sentence_without_checksum("$GPZDA,225239,14,11,2015,-00,30");
    w.write_text_sentence("Following 1 sentence is a valid ZDA message with a long second fraction");
    w.write_sentence_without_checksum("$GPZDA,225239.1234567890,14,11,2015,-00,30");
    w.write_text_sentence("Following 1 sentence is a incorrect ZDA message without leading zeros");
    w.write_sentence_without_checksum("$GPZDA,25239,4,1,15,5,0");
    w.write_text_sentence("Following 7 sentence is a incorrect ZDA message with no fields");
    w.write_sentence_without_checksum("$GPZDA,,,,,,");
    w.write_sentence_without_checksum("$GPZDA,,,,,");
    w.write_sentence_without_checksum("$GPZDA,,,,");
    w.write_sentence_without_checksum("$GPZDA,,,");
    w.write_sentence_without_checksum("$GPZDA,,");
    w.write_sentence_without_checksum("$GPZDA,");
}