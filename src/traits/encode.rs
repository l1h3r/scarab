use wasmlib::BytesDecoder;
use wasmlib::BytesEncoder;

pub trait Encode: Sized {
  fn decode(decoder: &mut BytesDecoder) -> Self;

  fn encode(&self, encoder: &mut BytesEncoder);

  fn to_bytes(&self) -> Vec<u8> {
    let mut encoder: BytesEncoder = BytesEncoder::new();

    self.encode(&mut encoder);

    encoder.data()
  }

  fn from_bytes(bytes: &[u8]) -> Self {
    Self::decode(&mut BytesDecoder::new(bytes))
  }
}
