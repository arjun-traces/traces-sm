#![no_main]
use libfuzzer_sys::fuzz_target;
use serde_json::Value;

fuzz_target!(|data: &[u8]| {
    // 1. Fuzz JSON-RPC arbitrary payload parsing
    if let Ok(val) = serde_json::from_slice::<Value>(data) {
        // Test deep recursion and field extraction
        if let Some(obj) = val.as_object() {
            let _ = obj.get("method");
            let _ = obj.get("params");
            let _ = obj.get("id");
            let _ = obj.get("name");
            let _ = obj.get("value");
        }

        // Test serialization round-trip
        let _ = serde_json::to_vec(&val);
    }
});
