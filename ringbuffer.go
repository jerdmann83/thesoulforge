package ringbuffer

import (
	"bytes"
	"errors"
	"fmt"
)

type RingBuffer struct {
	r, w int
	len  int
	buf  []byte
}

func (r *RingBuffer) Push(b byte) error {
	if r.len < cap(r.buf) {
		r.len += 1
	}
	if r.w >= cap(r.buf) {
		r.w = 0
	}
	r.buf[r.w] = b
	r.w += 1
	if r.w >= cap(r.buf) {
		r.w = 0
	}
	return nil
}

func (r *RingBuffer) Pop() (byte, error) {
	if r.len == 0 {
		return byte(0), errors.New("ringbuffer empty")
	}
	r.len -= 1
	res := r.buf[r.r]
	r.r += 1
	if r.r >= cap(r.buf) {
		r.r = 0
	}
	return res, nil
}

func (r *RingBuffer) String() string {
	var buffer bytes.Buffer
	for i := r.r; i != r.w; i++ {
		buffer.WriteByte(r.buf[i])
		if i >= cap(r.buf) {
			i = 0
		}
	}
	return buffer.String()
}

func New(size uint64) *RingBuffer {
	r := RingBuffer{
		0,
		0,
		0,
		make([]byte, size),
	}
	return &r
}

func main() {
	fmt.Println("vim-go")
}
