package main

import (
	"bytes"
	"encoding/binary"
	"io"
	"sync"
)

var bufferPool = sync.Pool{
	New: func() interface{} {
		return bytes.NewBuffer(nil)
	},
}

type Command uint8

const (
	CommandSubscribe Command = iota
	CommandUnsubscribe
	CommandPublish
	CommandRequest
	CommandResponse
	CommandError
)

type Message struct {
	Command Command
	Subject string
	Data    []byte
}

func (m *Message) Serialize(writer io.Writer) error {
	t := [4]byte{}

	buf := bufferPool.Get().(*bytes.Buffer)
	defer bufferPool.Put(buf)

	buf.Reset()

	buf.WriteByte(byte(m.Command))

	binary.BigEndian.PutUint16(t[:2], uint16(len(m.Subject)))
	buf.Write(t[:2])
	buf.WriteString(m.Subject)

	binary.BigEndian.PutUint32(t[:4], uint32(len(m.Data)))
	buf.Write(t[:4])
	buf.Write(m.Data)

	_, err := buf.WriteTo(writer)
	return err
}

func (m *Message) Deserialize(reader io.Reader) (*Message, error) {
	t := [4]byte{}

	if _, err := io.ReadFull(reader, t[:1]); err != nil {
		return nil, err
	}

	m.Command = Command(t[0])

	if _, err := io.ReadFull(reader, t[:2]); err != nil {
		return nil, err
	}

	subjectLength := binary.BigEndian.Uint16(t[:2])

	subject := make([]byte, subjectLength)
	if _, err := io.ReadFull(reader, subject); err != nil {
		return nil, err
	}

	m.Subject = string(subject)

	if _, err := io.ReadFull(reader, t[:4]); err != nil {
		return nil, err
	}

	dataLength := binary.BigEndian.Uint32(t[:4])

	data := make([]byte, dataLength)
	if _, err := io.ReadFull(reader, data); err != nil {
		return nil, err
	}

	m.Data = data

	return m, nil
}
