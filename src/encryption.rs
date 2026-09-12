use aes::cipher::{block_padding::Iso10126, BlockModeDecrypt, KeyIvInit};

// garupa uses Aes128 cbc encryption with Iso10126 padding

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

// Decrypts an encrypted byte array and returns the output, or an error
pub fn decrypt(key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, String> {
    let key: [u8; 16] = key.try_into().expect("aes key must be 16 bytes");
    let iv: [u8; 16] = iv.try_into().expect("aes iv must be 16 bytes");

    let decrypted_data = Aes128CbcDec::new(&key.into(), &iv.into())
          .decrypt_padded_vec::<Iso10126>(input)
          .unwrap();

    Ok(decrypted_data)
}
