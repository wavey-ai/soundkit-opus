use libopus::{decoder, encoder};
use soundkit::audio_packet::{Decoder, Encoder};
use std::ops::BitOr;

pub struct OpusEncoder {
    encoder: encoder::Encoder,
    sample_rate: u32,
    channels: u32,
    bits_per_sample: u32,
    frame_size: u32,
    bitrate: u32,
}

impl Encoder for OpusEncoder {
    fn new(
        sample_rate: u32,
        bits_per_sample: u32,
        channels: u32,
        frame_size: u32,
        bitrate: u32,
    ) -> Self {
        let encoder =
            encoder::Encoder::create(48000, 2, 1, 1, &[0u8, 1u8], encoder::Application::Audio)
                .unwrap();

        Self {
            encoder,
            sample_rate,
            channels,
            bits_per_sample,
            frame_size,
            bitrate,
        }
    }

    fn init(&mut self) -> Result<(), String> {
        self.reset()
    }

    fn encode_i16(&mut self, input: &[i16], output: &mut [u8]) -> Result<usize, String> {
        self.encoder
            .encode(input, output)
            .map_err(|e| e.to_string())
    }

    fn encode_i32(&mut self, input: &[i32], output: &mut [u8]) -> Result<usize, String> {
        Err("Not implemented.".to_string())
    }

    fn reset(&mut self) -> Result<(), String> {
        match self
            .encoder
            .set_option(encoder::OPUS_SET_BITRATE_REQUEST, self.bitrate as u32)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("error reseting opus: {}", e)),
        }
    }
}

pub struct OpusDecoder {
    decoder: decoder::Decoder,
}

impl Decoder for OpusDecoder {
    fn decode(&mut self, input: &[u8], output: &mut [i16], fec: bool) -> Result<usize, String> {
        self.decoder
            .decode(input, output, fec)
            .map_err(|e| e.to_string())
    }
}
