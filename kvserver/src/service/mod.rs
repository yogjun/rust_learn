use crate::{command_request::RequestData, *};

mod command_service;

/// 对command处理的抽象
pub trait CommandService {
    /// 处理command，返回response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}
