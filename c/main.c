#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct EntryList {
    char* key;
    char* value;
    struct EntryList* next;
} EntryList;

typedef struct {
    EntryList **buckets; // array of entries
    int capacity;   // size of buckets array
    int size; // number of key/value pairs stored
} HashMap;

unsigned int hash(char* key) {
    unsigned int h = 0;
    for (size_t i = 0; i < strlen(key); i++) {
        char c = key[i];
        h = (h * 31 + c); 
    }
    return (unsigned int) h;
}

HashMap* new_hashmap(int capacity) {
    EntryList **buckets = calloc(capacity, sizeof(EntryList *));

    HashMap *h = malloc(sizeof(HashMap));
    h->capacity = capacity;
    h->size = 0;
    h->buckets = buckets;

    return h;
}

void hashmap_cleanup(HashMap *map) {
    for (int i = 0; i < map->capacity; i++) {
        EntryList *current = map->buckets[i];
        while (current != NULL) {
            EntryList* next = current->next;
            free(current);
            current = next;
        }
    }
    free(map->buckets);
    free(map);
}

void hashmap_resize(HashMap *map) {
    int new_capacity = map->capacity * 2;
    EntryList **new_buckets = calloc(new_capacity, sizeof(EntryList *));

    // loop through each bucket
    for (int i = 0; i < map->capacity; i++) {
        EntryList *current = map->buckets[i];

        // loop through every entry in the current bucket (linked list)
        while (current != NULL) {
            // save off the next entry before we overwrite it
            EntryList *next_entry = current->next;
            // compute the new bucket index
            int new_index = hash(current->key) % new_capacity;

            // in case the bucket has entries, point our node at 
            // whatever is currently in there
            current->next = new_buckets[new_index];

            // make our `current` node the new head of the linked list
            new_buckets[new_index] = current;

            current = next_entry;
        }
    }

    free(map->buckets);

    map->buckets = new_buckets;
    map->capacity = new_capacity;
}

void hashmap_put(HashMap* map, char* key, char* value) {
    unsigned int index = hash(key) % map->capacity;

    EntryList *head = map->buckets[index];

    EntryList *current = head;
    while(current != NULL) {
        if (strcmp(current->key, key) == 0) {
            current->value = value;
            return;
        }
        current = current->next;
    } 

    EntryList *new_entry = malloc(sizeof(EntryList));
    new_entry->key = key;
    new_entry->value = value;
    new_entry->next = head;

    map->buckets[index] = new_entry;
    map->size += 1;

    // trigger resize if we are nearing capacity
    if ((float)map->size / map->capacity > 0.75f) {
        hashmap_resize(map);
    }
}

char* hashmap_get(HashMap *map, char* key) {
    unsigned int index = hash(key) % map->capacity;

    EntryList *current = map->buckets[index];
    while (current != NULL) {
        if (strcmp(current->key, key) == 0) {
            return current->value;
        }

        current = current->next;
    }

    return NULL;
}

void hashmap_delete(HashMap *map, char* key) {
    unsigned int index = hash(key) % map->capacity;

    EntryList *current = map->buckets[index];
    EntryList *prev = NULL;

    while (current != NULL) {
        if (strcmp(current->key, key) == 0) {
            if (prev == NULL) {
                map->buckets[index] = current->next;
            } else {
                prev->next = current->next;
            }

            map->size -= 1;
            return;
        }

        prev = current;
        current = current->next;
    }
}

int main(void) {
    HashMap *map = new_hashmap(100);

    hashmap_put(map, "name", "Noah");

    printf("hello, %s\n", hashmap_get(map, "name"));

    hashmap_cleanup(map);

    return 0;
}


