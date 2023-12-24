
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
    
    match message.reply {
        Some(reply) => {
            buffer.extend_from_slice(&reply.len().to_be_bytes());
            buffer.extend_from_slice(reply.as_slice());
        },
        None => {
            buffer.extend_from_slice(&0u64.to_be_bytes());
        },
    }

    Ok(buffer)
}

pub fn deserialize(data: Vec<u8>) -> Result<(Message, usize), Box<dyn std::error::Error>> {
    let mut cursor = 0 as usize;

    let subject_len = u64::from_be_bytes(data[cursor..cursor + 8].try_into()?);
    cursor += 8;
    let mut subject_buffer = vec![0; subject_len as usize];
    subject_buffer.copy_from_slice(&data[cursor..cursor + subject_len as usize]);
    cursor += subject_len as usize;

    let message_len = u64::from_be_bytes(data[cursor + subject_len as usize..cursor + subject_len as usize + 8].try_into()?);
    cursor += 8 + subject_len as usize;
    let mut message_buffer = vec![0; message_len as usize];
    message_buffer.copy_from_slice(&data[cursor..cursor + message_len as usize]);
    cursor += message_len as usize;

    let reply_len = u64::from_be_bytes(data[cursor + message_len as usize..cursor + message_len as usize + 8].try_into()?);
    cursor += 8 + message_len as usize;
    let mut reply_buffer = None;
    if reply_len > 0 {
        let mut buffer = vec![0; reply_len as usize];
        buffer.copy_from_slice(&data[cursor..cursor + reply_len as usize]);

        reply_buffer = Some(buffer);
        cursor += reply_len as usize;
    }

    Ok((Message {
        subject: String::from_utf8(subject_buffer)?,
        data: message_buffer,
        reply: reply_buffer,
    }, cursor))
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Message {
            subject: self.subject.clone(),
            data: self.data.clone(),
            reply: self.reply.clone(),
        }
    }
}