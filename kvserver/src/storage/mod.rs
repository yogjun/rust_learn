mod memory;
use crate::{KvError, Kvpair, Value};
pub use memory::MemTable;

/// 对存储的抽象
/// 不关心数据放在哪，但需要定义外界与存储打交道
pub trait Storage {
    /// 从一个 HashTable 里获取一个key的value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从一个 HashTable 里设置一个key的value，返回旧的value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// 从HashTable中删除一个key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 返回HashTable所有的kv pair dep
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    /// 返回HashTable的Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basi_interface(store);
    }

    fn test_basi_interface(store: impl Storage) {
        // 第一次 set 会创建 table，插入 key 并返回 None（之前没值）
        let v = store.set("t1", "hello".into(), "world".into());
        assert!(v.unwrap().is_none());
    }
}
