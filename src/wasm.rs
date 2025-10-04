use crate::base::EncodingPacket;
use crate::base::ObjectTransmissionInformation;
use crate::decoder::Decoder as DecoderNative;
use crate::encoder::Encoder as EncoderNative;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Encoder {
    encoder: EncoderNative,
}

#[wasm_bindgen]
impl Encoder {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8], maximum_transmission_unit: u16) -> Encoder {
        let encoder = EncoderNative::with_defaults(data, maximum_transmission_unit);
        Encoder { encoder }
    }

    pub fn get_encoded_packets(&self, repair_packets_per_block: u32) -> js_sys::Array {
        let packets = self
            .encoder
            .get_encoded_packets(repair_packets_per_block);
        let array = js_sys::Array::new();
        for packet in packets {
            let serialized = packet.serialize();
            let js_bytes = js_sys::Uint8Array::from(&serialized[..]);
            array.push(&js_bytes.into());
        }
        array
    }
}

#[wasm_bindgen]
pub struct Decoder {
    decoder: DecoderNative,
}

#[wasm_bindgen]
impl Decoder {
    #[wasm_bindgen(constructor)]
    pub fn new(transfer_length: u64, maximum_transmission_unit: u16) -> Decoder {
        let config =
            ObjectTransmissionInformation::with_defaults(transfer_length, maximum_transmission_unit);
        let decoder = DecoderNative::new(config);
        Decoder { decoder }
    }

    pub fn decode(&mut self, packet: &[u8]) -> Option<Vec<u8>> {
        self.decoder
            .decode(EncodingPacket::deserialize(packet))
    }
}