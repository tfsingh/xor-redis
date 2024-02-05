use redis_module::key::RedisKey;
use redis_module::native_types::RedisType;
use redis_module::{raw, redis_module, Context, NextArg, RedisError, RedisResult, RedisString};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::os::raw::c_void;

#[derive(Debug)]
struct Xor {
    seed: u64,
    block_length: usize,
    fingerprints: Vec<u8>,
}

impl Xor {
    const MAX_ITERATIONS: u32 = 1024;

    fn populate(entries: Vec<RedisString>) -> Self {
        unimplemented!();
    }

    fn contains(&self, entry: RedisString) -> bool {
        true
    }
}

static XOR_REDIS: RedisType = RedisType::new(
    "RedisXorFilter",
    0,
    raw::RedisModuleTypeMethods {
        version: raw::REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: None,
        rdb_save: None,
        aof_rewrite: None,
        free: Some(free),

        // Currently unused by Redis
        mem_usage: None,
        digest: None,

        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save2: None,
        aux_save_triggers: 0,

        free_effort: None,
        unlink: None,
        copy: None,
        defrag: None,

        copy2: None,
        free_effort2: None,
        mem_usage2: None,
        unlink2: None,
    },
);

unsafe extern "C" fn free(value: *mut c_void) {
    drop(Box::from_raw(value.cast::<Xor>()));
}

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
    let key = args.next_arg()?;
    let entry = args.next_arg()?;

    let key = ctx.open_key_writable(&key);

    if let Some(filter) = key.get_value::<Xor>(&XOR_REDIS)? {
        RedisResult::Ok(filter.contains(entry).into())
    } else {
        RedisResult::Err(redis_module::RedisError::Str("Filter not present"))
    }
}

redis_module! {
    name: "xor-redis",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [XOR_REDIS],
    commands: [
        ["xor.populate", populate, "write", 1, 1, 1],
        ["xor.contains", contains, "readonly", 1, 1, 1]
    ],
}
