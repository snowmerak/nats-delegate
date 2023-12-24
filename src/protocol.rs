
pub struct Message {
    pub subject: String,
    pub data: Vec<u8>,
    pub reply: Option<Vec<u8>>,
}

pub fn serialize(message: Message) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    buffer.extend_from_slice(&message.subject.len().to_be_bytes());
    buffer.extend_from_slice(message.subject.as_bytes());
    buffer.extend_from_slice(&message.data.len().to_be_bytes());
    buffer.extend_from_slice(message.data.as_slice());
    
    match reply {
        Some(reply) => {
            buffer.extend_from_slice(&message.reply.len().to_be_bytes());
            buffer.extend_from_slice(message.reply.as_slice());
        },
        None => {
            buffer.extend_from_slice(&0u64.to_be_bytes());
        },
    }

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

    let reply_len = cursor.get_u64();
    let mut reply_buffer = None;
    if reply_len > 0 {
        let mut reply_buffer = vec![0; reply_len as usize];
        cursor.read_exact(&mut reply_buffer)?;

        reply_buffer = Some(reply_buffer);
    }

    Ok(Message {
        subject: String::from_utf8(subject_buffer)?,
        data: message_buffer,
        reply: reply_buffer,
    })
}
