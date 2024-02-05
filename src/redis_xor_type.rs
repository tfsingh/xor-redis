use crate::xor::Xor;
use redis_module::native_types::RedisType;
use redis_module::raw;
use std::os::raw::c_void;

pub static XOR_REDIS: RedisType = RedisType::new(
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

pub unsafe extern "C" fn free(value: *mut c_void) {
    drop(Box::from_raw(value.cast::<Xor>()));
}
