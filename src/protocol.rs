
pub fn serialize(subject: &str, message: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    buffer.extend_from_slice(&subject.len().to_be_bytes());
    buffer.extend_from_slice(subject.as_bytes());
    buffer.extend_from_slice(&message.len().to_be_bytes());
    buffer.extend_from_slice(message.data.as_slice());

    Ok(buffer)
}

pub fn deserialize(data: Vec<u8>) -> Result<Message, Box<dyn std::error::Error>> {
    let mut cursor = std::io::Cursor::new(data);

    let subject_len = cursor.get_u64();
    let mut subject_buffer = vec![0; subject_len as usize];
    cursor.read_exact(&mut subject_buffer)?;

    let message_len = cursor.get_u64();
    let mut message_buffer = vec![0; message_len as usize];
    cursor.read_exact(&mut message_buffer)?;

    Ok(Message {
        subject: String::from_utf8(subject_buffer)?,
        data: message_buffer,
        reply: None,
    })
}
