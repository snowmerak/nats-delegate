package main

import (
	"fmt"
	"sync/atomic"
)

type Receiver struct {
	buffer   chan *Message
	length   atomic.Int64
	capacity int64
}

func NewReceiver(size int) *Receiver {
	return &Receiver{
		buffer:   make(chan *Message, size),
		length:   atomic.Int64{},
		capacity: int64(size),
	}
}

func (r *Receiver) TryPush(m *Message) error {
	if r.length.Load() >= r.capacity {
		return &ReceiverBufferFullError{currentLength: r.length.Load(), capacity: r.capacity}
	}

	b := false
	l := r.length.Load()
	for l < r.capacity {
		if r.length.CompareAndSwap(l, l+1) {
			b = true
			break
		}
		l = r.length.Load()
	}

	if !b {
		return &ReceiverBufferCannotInputError{currentLength: r.length.Load(), capacity: r.capacity}
	}

	r.buffer <- m

	return nil
}

func (r *Receiver) Push(m *Message) {
	for {
		l := r.length.Load()
		if l >= r.capacity {
			continue
		}
		if r.length.CompareAndSwap(l, l+1) {
			break
		}
	}

	r.buffer <- m
}

func (r *Receiver) TryPop() (*Message, error) {
	if r.length.Load() <= 0 {
		return nil, &ReceiverBufferEmptyError{currentLength: r.length.Load(), capacity: r.capacity}
	}

	b := false
	l := r.length.Load()
	for l > 0 {
		if r.length.CompareAndSwap(l, l-1) {
			b = true
			break
		}
		l = r.length.Load()
	}

	if !b {
		return nil, &ReceiverBufferCannotOutputError{currentLength: r.length.Load(), capacity: r.capacity}
	}

	return <-r.buffer, nil
}

func (r *Receiver) Pop() *Message {
	for {
		l := r.length.Load()
		if l <= 0 {
			continue
		}
		if r.length.CompareAndSwap(l, l-1) {
			break
		}
	}

	return <-r.buffer
}

type ReceiverBufferFullError struct {
	currentLength int64
	capacity      int64
}

func (e *ReceiverBufferFullError) Error() string {
	return fmt.Sprintf("receiver buffer full: %d/%d", e.currentLength, e.capacity)
}

type ReceiverBufferCannotOutputError struct {
	currentLength int64
	capacity      int64
}

func (e *ReceiverBufferCannotOutputError) Error() string {
	return fmt.Sprintf("receiver buffer cannot output: %d/%d", e.currentLength, e.capacity)
}

type ReceiverBufferCannotInputError struct {
	currentLength int64
	capacity      int64
}

type ReceiverBufferEmptyError struct {
	currentLength int64
	capacity      int64
}

func (e *ReceiverBufferEmptyError) Error() string {
	return fmt.Sprintf("receiver buffer empty: %d/%d", e.currentLength, e.capacity)
}

func (e *ReceiverBufferCannotInputError) Error() string {
	return fmt.Sprintf("receiver buffer cannot input: %d/%d", e.currentLength, e.capacity)
}

type Broadcaster struct {
}
