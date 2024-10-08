// Copyright (C) 2024 Casper Meijn <casper@meijn.net>
//
// SPDX-License-Identifier: CC0-1.0

struct UdpSocket {
    socket: std::net::UdpSocket,
}

impl UdpSocket {
    fn new() -> Self {
        Self {
            socket: std::net::UdpSocket::bind("127.0.0.1:2000").unwrap(),
        }
    }

    fn write_payload(&self, payload: &str) {
        self.socket
            .send_to(payload.as_bytes(), "127.0.0.1:2000")
            .unwrap();
    }

    fn write_sentence_without_checksum(&self, without_checksum: &str) {
        let checksum = without_checksum
            .chars()
            .skip(1)
            .fold(0, |sum, c| sum ^ c as u8);
        self.write_payload(&format!("{without_checksum}*{checksum:02x}\r\n"));
    }

    fn write_text_sentence(&self, text: &str) {
        self.write_sentence_without_checksum(&format!("$GPTXT,01,01,{text}"));
    }
}

fn main() {
    let w = UdpSocket::new();
    w.write_text_sentence("Next 1 sentence is a valid message");
    w.write_sentence_without_checksum("$GPGLC,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 3 sentence has invalid newline");
    w.write_payload("$GPGLC,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*42");
    w.write_payload("$GPGLC,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*42\r");
    w.write_payload("$GPGLC,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*42\n");
    w.write_text_sentence("Following 1 sentence has incorrect checksum");
    w.write_payload("$GPGLC,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*99\r\n");
    w.write_text_sentence("Following 4 sentence is missing a checksum");
    w.write_payload("$GPGLC,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*9\r\n");
    w.write_payload("$GPGLC,225239.784,2,N,01217.461,E,,5,3.5,,,,,,*\r\n");
    w.write_payload("$GPGLC,225239.784,2,N,01217.461,E,,5,3.5,,,,,,\r\n");
    w.write_payload("$GPGLC,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 1 sentence starts with invalid character");
    w.write_sentence_without_checksum("#GPGLC,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 1 sentence ha unknown talker id");
    w.write_sentence_without_checksum("$ZZGLC,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 1 sentence has unknown sentence id");
    w.write_sentence_without_checksum("$GPZZZ,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 1 sentence has too long sentence id");
    w.write_sentence_without_checksum("$GPZZZZZZ,225239.784,2,N,01217.461,E,,5,3.5,,,,,,");
    w.write_text_sentence("Following 2 sentence has too short sentence id");
    w.write_sentence_without_checksum("$GPGG");
    w.write_payload("$GPGG");
    w.write_text_sentence("Following 1 sentence has no fields");
    w.write_sentence_without_checksum("$GPGLC,");

    w.write_text_sentence("Following 1 sentence is a valid DPT message");
    w.write_sentence_without_checksum("$INDPT,2.3,0.0,300");
    w.write_text_sentence("Following 1 sentence is a valid GGA message with only empty fields");
    w.write_sentence_without_checksum("$INDPT,,,");
    w.write_text_sentence("Following sentences is a incorrect DPT message missing fields");
    w.write_sentence_without_checksum("$INDPT,,");
    w.write_sentence_without_checksum("$INDPT,");

    w.write_text_sentence("Following 1 sentence is a valid HDT message");
    w.write_sentence_without_checksum("$GPHDT,274.07,T");
    w.write_text_sentence("Following 1 sentence is a valid HDT message without fraction");
    w.write_sentence_without_checksum("$GPHDT,274,T");
    w.write_text_sentence("Following 1 sentence is a valid HDT message with long fraction");
    w.write_sentence_without_checksum("$GPHDT,274.12345678,T");
    w.write_text_sentence("Following 1 sentence is a valid HDT message with only empty fields");
    w.write_sentence_without_checksum("$GPHDT,,");
    w.write_text_sentence("Following 1 sentence is a invalid HDT message with incorrect unit");
    w.write_sentence_without_checksum("$GPHDT,274.07,Z");
    w.write_text_sentence("Following sentences is a incorrect HDT message missing fields");
    w.write_sentence_without_checksum("$GPHDT,");

    w.write_text_sentence("Following 1 sentence is a valid GGA message");
    w.write_sentence_without_checksum(
        "$GNGGA,001043.00,4404.14036,N,12118.85961,W,1,12,0.98,1113.0,M,-21.3,M,123,45",
    );
    w.write_text_sentence("Following 1 sentence is a valid GGA message with only empty fields");
    w.write_sentence_without_checksum("$GNGGA,,,,,,,,,,,,,,");
    w.write_text_sentence("Following 1 sentence is a valid GGA message without second fraction");
    w.write_sentence_without_checksum(
        "$GNGGA,120043,4404.14036,N,12118.85961,W,1,12,0.98,1113.0,M,-21.3,M,123,45",
    );
    w.write_text_sentence(
        "Following 1 sentence is a valid GGA message without lat/long minute fraction",
    );
    w.write_sentence_without_checksum(
        "$GNGGA,120043,4404,N,12118,W,1,12,0.98,1113.0,M,-21.3,M,123,0045",
    );
    w.write_text_sentence(
        "Following 1 sentence is a incorrect GGA message without lat/long direction",
    );
    w.write_sentence_without_checksum(
        "$GNGGA,120043,4404,,12118,,1,12,0.98,1113.0,M,-21.3,M,123,0045",
    );
    w.write_text_sentence("Following 1 sentence is a incorrect GGA message invalid units");
    w.write_sentence_without_checksum(
        "$GNGGA,001043.00,4404.14036,N,12118.85961,W,1,12,0.98,1113.0,Z,-21.3,Z,123,45",
    );
    w.write_text_sentence("Following 1 sentence is a incorrect GGA message invalid quality");
    w.write_sentence_without_checksum(
        "$GNGGA,001043.00,4404.14036,N,12118.85961,W,9,12,0.98,1113.0,M,-21.3,M,123,45",
    );
    w.write_text_sentence("Following 13 sentence is a incorrect GGA message missing fields");
    w.write_sentence_without_checksum("$GNGGA,,,,,,,,,,,,,");
    w.write_sentence_without_checksum("$GNGGA,,,,,,,,,,,,");
    w.write_sentence_without_checksum("$GNGGA,,,,,,,,,,,");
    w.write_sentence_without_checksum("$GNGGA,,,,,,,,,,");
    w.write_sentence_without_checksum("$GNGGA,,,,,,,,,");
    w.write_sentence_without_checksum("$GNGGA,,,,,,,,");
    w.write_sentence_without_checksum("$GNGGA,,,,,,,");
    w.write_sentence_without_checksum("$GNGGA,,,,,,");
    w.write_sentence_without_checksum("$GNGGA,,,,,");
    w.write_sentence_without_checksum("$GNGGA,,,,");
    w.write_sentence_without_checksum("$GNGGA,,,");
    w.write_sentence_without_checksum("$GNGGA,,");
    w.write_sentence_without_checksum("$GNGGA,");

    w.write_text_sentence("Following 1 sentence is a valid GLL message");
    w.write_sentence_without_checksum("$GNGLL,4404.14012,N,12118.85993,W,001037.00,A,A");
    w.write_text_sentence("Following 4 sentence is a valid GLL message with several FAA mode");
    w.write_sentence_without_checksum("$GNGLL,4404.14012,N,12118.85993,W,001037.00,A,S");
    w.write_sentence_without_checksum("$GNGLL,4404.14012,N,12118.85993,W,001037.00,A,C");
    w.write_sentence_without_checksum("$GNGLL,4404.14012,N,12118.85993,W,001037.00,A,F");
    w.write_sentence_without_checksum("$GNGLL,4404.14012,N,12118.85993,W,001037.00,A,N");
    w.write_text_sentence("Following 1 sentence is a valid GLL message with status = void");
    w.write_sentence_without_checksum("$GNGLL,4404.14012,N,12118.85993,W,001037.00,V,A");
    w.write_text_sentence("Following 1 sentence is a valid GLL message with only empty fields");
    w.write_sentence_without_checksum("$GNGLL,,,,,,,");
    w.write_text_sentence("Following 1 sentence is a valid GLL message without second fraction");
    w.write_sentence_without_checksum("$GNGLL,4404.14012,N,12118.85993,W,001037,A,A");
    w.write_text_sentence(
        "Following 1 sentence is a valid GLL message without lat/long minute fraction",
    );
    w.write_sentence_without_checksum("$GNGLL,4404,N,12118,W,001037.00,A,A");
    w.write_text_sentence(
        "Following 1 sentence is a incorrect GLL message without lat/long direction",
    );
    w.write_sentence_without_checksum("$GNGLL,4404.14012,,12118.85993,,001037.00,A,A");
    w.write_text_sentence("Following 1 sentence is a incorrect GLL message invalid status");
    w.write_sentence_without_checksum("$GNGLL,4404.14012,N,12118.85993,W,001037.00,Z,A");
    w.write_text_sentence("Following 1 sentence is a incorrect GLL message invalid FAA mode");
    w.write_sentence_without_checksum("$GNGLL,4404.14012,N,12118.85993,W,001037.00,A,Z");
    w.write_text_sentence("Following sentences is a incorrect GLL message missing fields");
    w.write_sentence_without_checksum("$GNGLL,,,,,,");
    w.write_sentence_without_checksum("$GNGLL,,,,,");
    w.write_sentence_without_checksum("$GNGLL,,,,");
    w.write_sentence_without_checksum("$GNGLL,,,");
    w.write_sentence_without_checksum("$GNGLL,,");
    w.write_sentence_without_checksum("$GNGLL,");

    w.write_text_sentence("Following 1 sentence is a valid GST message");
    w.write_sentence_without_checksum("$GPGST,182141.000,15.5,15.3,7.2,21.8,0.9,0.5,0.8");
    w.write_text_sentence("Following 1 sentence is a GST message without fraction");
    w.write_sentence_without_checksum("$GPGST,182141,15,15,7,21,0,0,0");
    w.write_text_sentence("Following 1 sentence is valid GST message with empty fields");
    w.write_sentence_without_checksum("$GPGST,,,,,,,,");
    w.write_text_sentence("Following sentences are incorrect GST message missing fields");
    w.write_sentence_without_checksum("$GPGST,,,,,,,");
    w.write_sentence_without_checksum("$GPGST,,,,,,");
    w.write_sentence_without_checksum("$GPGST,,,,,");
    w.write_sentence_without_checksum("$GPGST,,,,");
    w.write_sentence_without_checksum("$GPGST,,,");
    w.write_sentence_without_checksum("$GPGST,,");
    w.write_sentence_without_checksum("$GPGST,");

    w.write_text_sentence("Following 1 sentence is a valid ROT message");
    w.write_sentence_without_checksum("$GPROT,35.6,A");
    w.write_text_sentence("Following 1 sentence is a valid ROT message with status = void");
    w.write_sentence_without_checksum("$GPROT,,V");
    w.write_text_sentence("Following 1 sentence is a valid ROT message with only empty fields");
    w.write_sentence_without_checksum("$GPROT,,");
    w.write_text_sentence("Following 1 sentence is a valid ROT message without fraction");
    w.write_sentence_without_checksum("$GPROT,35,A");
    w.write_text_sentence("Following 1 sentence is a valid ROT message with long fraction");
    w.write_sentence_without_checksum("$GPROT,35.12345678,A");
    w.write_text_sentence("Following sentences is a incorrect ROT message missing fields");
    w.write_sentence_without_checksum("$GPROT,");

    w.write_text_sentence("Following sentence is valid VBW message");
    w.write_sentence_without_checksum("$GPVBW,1.2,2.3,A,3.4,4.5,A,5.6,A,6.7,A");
    w.write_text_sentence("Following sentence is valid VBW without fraction");
    w.write_sentence_without_checksum("$GPVBW,1,2,A,3,4,A,5,A,6,A");
    w.write_text_sentence("Following sentence is valid VBW with long fraction");
    w.write_sentence_without_checksum(
        "$GPVBW,1.1234567,2.1234567,A,3.1234567,4.1234567,A,5.1234567,A,6.1234567,A",
    );
    w.write_text_sentence("Following sentence is valid VBW with only empty fields");
    w.write_sentence_without_checksum("$GPVBW,,,,,,,,,,");
    w.write_text_sentence("Following sentence is correct VBW message with invalid values");
    w.write_sentence_without_checksum("$GPVBW,1.2,2.3,V,3.4,4.5,V,5.6,V,6.7,V");
    w.write_text_sentence("Following sentences are incorrect VBW missing fields");
    w.write_sentence_without_checksum("$GPVBW,1.2,2.3,A,3.4,4.5,A,5.6,A,6.7");
    w.write_sentence_without_checksum("$GPVBW,1.2,2.3,A,3.4,4.5,A,5.6,A");
    w.write_sentence_without_checksum("$GPVBW,1.2,2.3,A,3.4,4.5,A,5.6");
    w.write_sentence_without_checksum("$GPVBW,1.2,2.3,A,3.4,4.5,A");
    w.write_sentence_without_checksum("$GPVBW,1.2,2.3,A,3.4,4.5");
    w.write_sentence_without_checksum("$GPVBW,1.2,2.3,A,3.4");
    w.write_sentence_without_checksum("$GPVBW,1.2,2.3,A");
    w.write_sentence_without_checksum("$GPVBW,1.2,2.3");
    w.write_sentence_without_checksum("$GPVBW,1.2");
    w.write_sentence_without_checksum("$GPVBW,");

    w.write_text_sentence("Following 1 sentence is a valid VHW message");
    w.write_sentence_without_checksum("$GPVHW,35.6,T,35.8,M,5.0,N,9.26,K");
    w.write_text_sentence("Following sentence is valid VHW without fraction");
    w.write_sentence_without_checksum("$GPVHW,35,T,35,M,5,N,9,K");
    w.write_text_sentence("Following sentence is valid VHW with long fraction");
    w.write_sentence_without_checksum("$GPVHW,35.1234567,T,35.2345678,M,5.3456789,N,9.4567890,K");
    w.write_text_sentence("Following sentence is valid VHW with only empty fields");
    w.write_sentence_without_checksum("$GPVHW,,,,,,,,");
    w.write_text_sentence("Following sentence is invalid VHW with incorrect units");
    w.write_sentence_without_checksum("$GPVHW,35,A,35,B,5,Y,9,Z");
    w.write_text_sentence("Following sentences are incorrect VHW missing fields");
    w.write_sentence_without_checksum("$GPVHW,35,T,35,M,5,N,9");
    w.write_sentence_without_checksum("$GPVHW,35,T,35,M,5,N");
    w.write_sentence_without_checksum("$GPVHW,35,T,35,M,5");
    w.write_sentence_without_checksum("$GPVHW,35,T,35,M");
    w.write_sentence_without_checksum("$GPVHW,35,T,35");
    w.write_sentence_without_checksum("$GPVHW,35,T");
    w.write_sentence_without_checksum("$GPVHW,35");
    w.write_sentence_without_checksum("$GPVHW,");

    w.write_text_sentence("Following sentence is valid VLW message");
    w.write_sentence_without_checksum("$GPVLW,1.2,N,2.3,N,3.4,N,4.5,N");
    w.write_text_sentence("Following sentence is valid VLW without fraction");
    w.write_sentence_without_checksum("$GPVLW,1,N,2,N,3,N,4,N");
    w.write_text_sentence("Following sentence is valid VHW with long fraction");
    w.write_sentence_without_checksum("$GPVLW,1.1234567,N,2.1234567,N,3.1234567,N,4.1234567,N");
    w.write_text_sentence("Following sentence is valid VHW with only empty fields");
    w.write_sentence_without_checksum("$GPVLW,,,,,,,,");
    w.write_text_sentence("Following sentence is invalid VHW with incorrect units");
    w.write_sentence_without_checksum("$GPVLW,1,A,2,B,3,Y,4,Z");
    w.write_text_sentence("Following sentences are incorrect VHW missing fields");
    w.write_sentence_without_checksum("$GPVLW,1,N,2,N,3,N,4");
    w.write_sentence_without_checksum("$GPVLW,1,N,2,N,3,N");
    w.write_sentence_without_checksum("$GPVLW,1,N,2,N,3");
    w.write_sentence_without_checksum("$GPVLW,1,N,2,N");
    w.write_sentence_without_checksum("$GPVLW,1,N,2");
    w.write_sentence_without_checksum("$GPVLW,1,N");
    w.write_sentence_without_checksum("$GPVLW,1");
    w.write_sentence_without_checksum("$GPVLW,");

    w.write_text_sentence("Following 1 sentence is a valid VTG message");
    w.write_sentence_without_checksum("$GPVTG,35.6,T,35.8,M,5.0,N,9.26,K,A");
    w.write_text_sentence("Following sentence is valid VTG without fraction");
    w.write_sentence_without_checksum("$GPVTG,35,T,35,M,5,N,9,K,A");
    w.write_text_sentence("Following sentence is valid VTG with long fraction");
    w.write_sentence_without_checksum("$GPVTG,35.1234567,T,35.2345678,M,5.3456789,N,9.4567890,K,A");
    w.write_text_sentence("Following sentence is valid VTG with only empty fields");
    w.write_sentence_without_checksum("$GPVTG,,,,,,,,,");
    w.write_text_sentence("Following 4 sentence is valid VTG with several FAA mode");
    w.write_sentence_without_checksum("$GPVTG,35,T,35,M,5,N,9,K,S");
    w.write_sentence_without_checksum("$GPVTG,35,T,35,M,5,N,9,K,C");
    w.write_sentence_without_checksum("$GPVTG,35,T,35,M,5,N,9,K,F");
    w.write_sentence_without_checksum("$GPVTG,35,T,35,M,5,N,9,K,N");
    w.write_text_sentence("Following sentence is incorrect VTG invalid FAA mode");
    w.write_sentence_without_checksum("$GPVTG,35,T,35,M,5,N,9,K,Z");
    w.write_text_sentence("Following sentence is invalid VTG with incorrect units");
    w.write_sentence_without_checksum("$GPVTG,35,A,35,B,5,Y,9,Z,A");
    w.write_text_sentence("Following sentences are incorrect VTG missing fields");
    w.write_sentence_without_checksum("$GPVTG,35,T,35,M,5,N,9,Z");
    w.write_sentence_without_checksum("$GPVTG,35,T,35,M,5,N,9");
    w.write_sentence_without_checksum("$GPVTG,35,T,35,M,5,N");
    w.write_sentence_without_checksum("$GPVTG,35,T,35,M,5");
    w.write_sentence_without_checksum("$GPVTG,35,T,35,M");
    w.write_sentence_without_checksum("$GPVTG,35,T,35");
    w.write_sentence_without_checksum("$GPVTG,35,T");
    w.write_sentence_without_checksum("$GPVTG,35");
    w.write_sentence_without_checksum("$GPVTG,");

    w.write_text_sentence("Following 1 sentence is a valid ZDA message");
    w.write_sentence_without_checksum("$GPZDA,225239.91,14,11,2015,+01,00");
    w.write_text_sentence("Following 1 sentence is a valid ZDA message with only empty fields");
    w.write_sentence_without_checksum("$GPZDA,,,,,,");
    w.write_text_sentence("Following 1 sentence is a valid ZDA message without second fraction");
    w.write_sentence_without_checksum("$GPZDA,225239,14,11,2015,+01,00");
    w.write_text_sentence("Following 1 sentence is a valid ZDA message with negative timezone");
    w.write_sentence_without_checksum("$GPZDA,225239,14,11,2015,-07,30");
    w.write_text_sentence(
        "Following 1 sentence is a valid ZDA message with negative zero timezone",
    );
    w.write_sentence_without_checksum("$GPZDA,225239,14,11,2015,-00,30");
    w.write_text_sentence(
        "Following 1 sentence is a valid ZDA message with a long second fraction",
    );
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
