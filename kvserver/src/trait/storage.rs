// /// 对存储的抽象
// /// 不关心数据放在哪，但需要定义外界与存储打交道
// pub trait Storage {
//     /// 从一个 HashTable 里获取一个key的value
//     fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
//     /// 从一个 HashTable 里设置一个key的value，返回旧的value
//     fn set(&self, table: &str, key: &str, value: &str) -> Result<Option<Value>, KvError>;
//     /// 查看 HashTable 中是否有 key
//     fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
//     /// 从HashTable中删除一个key
//     fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
//     /// 返回HashTable所有的kv pair dep
//     fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
//     /// 返回HashTable的Iterator
//     fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
// }
