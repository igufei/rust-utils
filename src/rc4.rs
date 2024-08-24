use crypto::rc4::Rc4;
use crypto::symmetriccipher::SynchronousStreamCipher;

/// 使用 RC4 加密数据
fn encode_rc4(key: &str, data: String) -> String {
    let key_bytes = key.as_bytes();
    let data_bytes = data.as_bytes();

    let mut encryptor = Rc4::new(key_bytes);
    let mut ciphertext = vec![0; data_bytes.len()];
    encryptor.process(data_bytes, &mut ciphertext);

    // 将加密后的字节数组转换为 Base64 编码的字符串
    base64::encode(&ciphertext)
}

/// 使用 RC4 解密数据
fn decode_rc4(key: &str, data: String) -> String {
    let key_bytes = key.as_bytes();

    // 将 Base64 编码的字符串解码为字节数组
    let data_bytes = base64::decode(&data).expect("Failed to decode base64 data");

    let mut decryptor = Rc4::new(key_bytes);
    let mut decrypted_text = vec![0; data_bytes.len()];
    decryptor.process(&data_bytes, &mut decrypted_text);

    // 将解密后的字节数组转换为字符串
    String::from_utf8(decrypted_text).expect("Failed to convert decrypted bytes to string")
}

fn main() {
    // 定义密钥
    let key = "supersecretkey";

    // 定义明文数据
    let plaintext = String::from("Hello, RC4 encryption!");

    // 加密数据
    let ciphertext = encode_rc4(key, plaintext.clone());
    println!("Ciphertext: {}", ciphertext);

    // 解密数据
    let decrypted_text = decode_rc4(key, ciphertext);
    println!("Decrypted text: {}", decrypted_text);
}