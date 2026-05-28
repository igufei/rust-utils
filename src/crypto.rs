use crypto::rc4::Rc4;
use crypto::symmetriccipher::SynchronousStreamCipher;
use crypto::{digest::Digest, sha1::Sha1};
pub fn sha1(input_str: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(input_str);
    let hex = hasher.result_str();
    hex
}

pub fn rc4_encode(key: &str, data: &str) -> String {
    let key_bytes = key.as_bytes();
    let data_bytes = data.as_bytes();

    let mut encryptor = Rc4::new(key_bytes);
    let mut ciphertext = vec![0; data_bytes.len()];
    encryptor.process(data_bytes, &mut ciphertext);

    // 将加密后的字节数组转换为 Base64 编码的字符串
    base64::encode(&ciphertext)
}

/// 使用 RC4 解密数据
pub fn rc4_decode(key: &str, data: &str) -> Result<String, String> {
    let key_bytes = key.as_bytes();

    // 将 Base64 编码的字符串解码为字节数组
    let data_bytes = base64::decode(data).map_err(|_| "rc4 base64 error".to_string())?;

    let mut decryptor = Rc4::new(key_bytes);
    let mut decrypted_text = vec![0; data_bytes.len()];
    decryptor.process(&data_bytes, &mut decrypted_text);

    // 将解密后的字节数组转换为字符串
    match String::from_utf8(decrypted_text) {
        Ok(text) => Ok(text),
        Err(_) => Err("rc4 decode error".to_string()),
    }
}
