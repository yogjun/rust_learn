
// 对command 的抽象处理
pub trait CommandService {
    // 处理Command,返回Response
    fn execute(self, store: &impl::Storage) -> Commandresponse;
}

// 从Request中得到Response ， 目前处理HGET/HGETALL/HSET
