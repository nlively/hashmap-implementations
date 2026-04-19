#[derive(Clone)]
struct HashMapEntry {
    key: String,
    value: String,
    // because this uses Box<>, it's heap allocated and `next` is implicitly
    // a pointer
    next: Option<Box<HashMapEntry>>,
}

struct HashMap {
    capacity: usize,
    size: usize,
    buckets: Vec<Option<Box<HashMapEntry>>>,
}

fn hash(value: &str) -> usize {
    let mut h: usize = 0;
    for i in value.chars() {
        h = h*32 + (i as usize);
    }
    h
}

impl HashMap {
    fn new(capacity: usize) -> Self {
        HashMap { 
            capacity, 
            size: 0, 
            buckets: vec![None; capacity],
        }
    }

    fn put(&mut self, key: &str, value: &str) {
        let index = hash(key) % self.capacity;

        let mut current = &mut self.buckets[index];

        while let Some(node) = current {
            if node.key == key {
                node.value = value.to_string();
                return;
            }
            current = &mut node.next;
        }

        let new_entry = HashMapEntry {
            key: key.to_string(),
            value: value.to_string(),
            next: None,
        };
        *current = Some(Box::new(new_entry));
        self.size += 1;

        if (self.size as f32) / (self.capacity as f32) > 0.75 {
            println!("hashmap has reached {}, at least 75 percent of its capacity", self.size);
            self.resize();
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        let index = hash(key) % self.capacity;

        let mut current = &self.buckets[index];

        while let Some(node) = current {
            if node.key == key {
                return Some(node.value.to_string());
            }

            current = &node.next;
        }

        return None
    }

    fn delete(&mut self, key: &str) {
        let index = hash(key) % self.capacity;
        let mut current = &mut self.buckets[index];

        loop {
            match current {
                None => return,
                Some(node) if node.key == key => {
                    *current = node.next.take();
                    self.size -= 1;
                    return;
                },
                Some(node) => current = &mut node.next,
            }
        }
    }

    fn dump(&self) {
        println!("HashMap::dump()");
        for bucket in self.buckets.iter() {
            let mut current = bucket.as_deref();
            while let Some(node) = current {
                println!("{}: {}", node.key, node.value);
                current = node.next.as_deref();
            }
        }
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;

        // a (mutable) new list of linked lists
        let mut new_buckets: Vec<Option<Box<HashMapEntry>>> = vec![None; new_capacity];

        // iterate through each bucket (each bucket is a linked list), 
        // pointing `current` at the head of the list
        // we use into_iter() to consume the old vector as we build the new one.
        // as such, we own `bucket` inside the loop
        for bucket in self.buckets.drain(..) {
            // point `current` at the head of the linked list
            let mut current = bucket;
            // iterate through every item in the linked list, and move ownership
            // into `node`
            while let Some(mut node) = current {
                // grab a the next item in the current
                // linked list so we can use it later.
                // now next_entry has ownership and node.next is empty
                let next_entry = node.next.take();

                // compute the new hash index based on the resized capacity
                let new_index = hash(node.key.as_str()) % new_capacity;

                // Link our node to whatever is at the head of the new bucket.
                // (might be Some, might be None).
                // we had previously moved the value out of node.next so now it has None
                node.next = new_buckets[new_index].take();

                // insert node into new_buckets
                new_buckets[new_index] = Some(node);

                // advance to the next item in the linked list
                // (move that value we took out of node.next into current)
                current = next_entry;
            }
        }

        self.buckets = new_buckets;
        self.capacity = new_capacity;
    }
}

fn print_name(m: &HashMap) {
    match &m.get("name") {
        Some(name) => println!("hello, {}", name),
        None => println!("no name found in hashmap"),
    };
}

fn main() {
    let mut m = HashMap::new(100);

    m.put("name", "Noah");
    print_name(&m);

    m.put("name", "Evan");
    print_name(&m);

    m.delete("name");
    print_name(&m);

    for i in 0..120 {
        let key = format!("item{}", i);
        let val = format!("value{}", i);
        m.put(key.as_str(), val.as_str());
    }

    m.dump();
}
