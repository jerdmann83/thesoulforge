package ringbuffer

import (
	"fmt"
	"testing"
)

func TestBasics(t *testing.T) {
	adds := 80
	reads := 10
	offset := 65
	r := New(100)
	for i := offset; i < adds+offset; i++ {
		r.Push(byte(i))
	}
	if r.len != adds {
		t.Error("unexpected len %i", r.len)
	}

	for i := 0; i < reads; i++ {
		r.Pop()
	}

	remaining := adds - reads
	if r.len != remaining {
		t.Error("unexpected remaining %i", r.len)
	}

	fmt.Printf("RingBuffer contents: %v\n", r.String())
}

func TestOverflow(t *testing.T) {
	adds := 99
	offset := 0
	size := 10
	r := New(10)
	for i := offset; i < adds+offset; i++ {
		r.Push(byte(i))
	}
	if r.len != size {
		t.Error("unexpected len %i", r.len)
	}

	out := make([]byte, size)
	for i := 0; i < size; i++ {
		next, err := r.Pop()
		if err != nil {
			t.Error("unexpected Pop failure")
		}
		out = append(out, next)
	}

	if r.len != 0 {
		t.Error("unexpected remaining %i", r.len)
	}
	_, ok := r.Pop()
	if ok == nil {
		t.Error("unexpected Pop success after emptying")
	}

	fmt.Printf("RingBuffer contents: %v\n", r.String())
}
