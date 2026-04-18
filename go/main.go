package main

import "fmt"

type Entry struct {
	key   string
	value string
	next  *Entry
}

type HashMap struct {
	capacity uint
	size     uint
	buckets  []*Entry
}

func NewHashMap(capacity uint) *HashMap {
	return &HashMap{
		capacity: capacity,
		size:     0,
		buckets:  make([]*Entry, capacity),
	}
}

func (m *HashMap) resize() {
	new_capacity := m.capacity * 2

	new_buckets := make([]*Entry, new_capacity)

	for _, current := range m.buckets {
		for current != nil {
			next_entry := current.next
			new_index := hash(current.key) % new_capacity

			current.next = new_buckets[new_index]
			new_buckets[new_index] = current

			current = next_entry
		}
	}

	m.buckets = new_buckets
	m.capacity = new_capacity
}

func (m *HashMap) put(key string, value string) {
	index := hash(key) % m.capacity

	head := m.buckets[index]
	current := head
	for current != nil {
		if current.key == key {
			current.value = value
			return
		}
		current = current.next
	}

	new_entry := &Entry{
		key:   key,
		value: value,
		next:  head,
	}
	m.buckets[index] = new_entry
	m.size += 1

	if float32(m.size)/float32(m.capacity) > 0.75 {
		m.resize()
	}
}

func (m *HashMap) get(key string) *string {
	index := hash(key) % m.capacity

	head := m.buckets[index]
	current := head
	for current != nil {
		if current.key == key {
			return &current.value
		}
	}

	return nil
}

func (m *HashMap) delete(key string) {
	index := hash(key) % m.capacity

	head := m.buckets[index]
	var prev *Entry
	current := head
	for current != nil {
		if current.key == key {
			if prev == nil {
				m.buckets[index] = prev
			} else {
				prev.next = current.next
			}

			m.size -= 1
			return
		}

		prev = current
		current = current.next
	}
}

func hash(value string) uint {
	var h uint = 0
	for _, c := range value {
		h = h*31 + uint(c)
	}
	return h
}

func main() {
	m := NewHashMap(100)

	m.put("name", "Noah")

	v := m.get("name")
	fmt.Printf("hello, %s\n", *v)
}
