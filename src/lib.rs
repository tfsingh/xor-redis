use redis_module::{redis_module, Context, NextArg, RedisResult, RedisString};
use redis_xor_type::XOR_REDIS;
use xor::Xor;

mod redis_xor_type;
mod xor;

fn populate(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let key = ctx.open_key_writable(&key);

    let entries: Vec<String> = args.map(|arg| arg.to_string()).collect();

    if let Some(filter) = key.get_value::<Xor>(&XOR_REDIS)? {
        match filter.populate(entries) {
            Ok(_) => RedisResult::Ok("Created filter".into()),
            Err(_) => RedisResult::Err(redis_module::RedisError::Str("Error in filter creation")),
        }
    } else {
        RedisResult::Err(redis_module::RedisError::Str("Filter not present"))
    }
}

fn contains(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_arg()?;
    let entry = args.next_arg()?;

    let key = ctx.open_key_writable(&key);

    if let Some(filter) = key.get_value::<Xor>(&XOR_REDIS)? {
        RedisResult::Ok(filter.contains(entry.to_string()).into())
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
