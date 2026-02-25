unsigned long hash_fnv1a(const char *str);
unsigned long hash_djb2(const char *str);
unsigned long hash_sdbm(const char *str);

struct hash_context;
typedef struct hash_context hash_context_t;

typedef double val_t;
hash_context_t* hash_context_init(unsigned long num);
void hash_on_val(hash_context_t* ctx, const char* loc, val_t val);
void hash_print_slots(hash_context_t* ctx);
