use redis_module::key::RedisKey;
use redis_module::{redis_module, Context, NextArg, RedisError, RedisResult, RedisString};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn fingerprint(hash: u64) -> u64 {
    unimplemented!()
}

fn hash_entry(entry: &RedisString) -> u64 {
    let mut hasher = DefaultHasher::new();
    entry.hash(&mut hasher);
    hasher.finish()
}

fn populate(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    RedisResult::Ok("not implemented".into())
}

fn contains(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);

    RedisResult::Ok("not implemented".into())
}

redis_module! {
    name: "xor-redis",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [],
    commands: [
        ["xor.populate", populate, "write", 1, 1, 1],
        ["xor.contains", contains, "readonly", 1, 1, 1]
    ],
}
