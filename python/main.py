class HashMapEntry:
    def __init__(self, key, value):
        self.key = key
        self.value = value
        self.next = None

def hash(value):
    h = 0

    for char in value:
        h = h*31 + ord(char)

    return h


class HashMap:
    def __init__(self, capacity):
        self.capacity = capacity
        self.size = 0
        self.buckets = [None] * capacity

    def put(self, key, value):
        index = hash(key) % self.capacity

        current = self.buckets[index]
        
        while current is not None:
            if current.key == key:
                current.value = value
                return
            
            current = current.next

        entry = HashMapEntry(key, value)
        entry.next = self.buckets[index]
        self.buckets[index] = entry

        self.size += 1

        if self.size / self.capacity > 0.75:
            print("hashmap has reached", self.size, ", exceeding 75 pct of capacity")
            self.resize()

    def get(self, key):
        index = hash(key) % self.capacity

        current = self.buckets[index]

        while current is not None:
            if current.key == key:
                return current.value
            current = current.next

    def delete(self, key):
        index = hash(key) % self.capacity

        current = self.buckets[index]
        prev = None

        while current is not None:
            if current.key == key:
                if prev is not None:
                    prev.next = current.next
                else:
                    self.buckets[index] = current.next
                
                self.size -= 1
                return
            
            prev = current
            current = current.next

    def resize(self):
        new_capacity = self.capacity * 2
        new_buckets = [None] * new_capacity

        for current in self.buckets:
            while current is not None:
                next_entry = current.next
                new_index = hash(current.key) % new_capacity

                current.next = new_buckets[new_index]
                new_buckets[new_index] = current

                current = next_entry

        self.buckets = new_buckets
        self.capacity = new_capacity

    def dump(self):
        print("HashMap.dump()")
        for current in self.buckets:
            while current is not None:
                print(f"{current.key}: {current.value}")
                current = current.next



def main():
    m = HashMap(100)

    def print_name():
        name = m.get("name")
        print("hello,", name)

    m.put("name", "Noah")
    print_name()

    m.put("name", "Evan")
    print_name()

    m.delete("name")
    print_name()

    for i in range(120):
        key = f"item{i}"
        val = f"value{i}"
        m.put(key, val)

    m.dump()


if __name__ == "__main__":
    main()