use std::collections::VecDeque;
use std::fmt::Formatter;

pub use cscan::CSCAN;
pub use fcfs::FCFS;
pub use scan::SCAN;
pub use sstf::SSTF;

mod cscan; /* 循环扫描算法 */
mod fcfs; /* 先来先服务算法 */
mod scan; /* 扫描算法 */
mod sstf; /* 短寻道时间优先算法 */

// Disk scheduler
/* 系统结构：
    操作系统
       | <- 请求块IO缓存层，传递地址、设备号
   块IO缓存层
       | <- 块IO层Push请求到读取队列尾，待当前读取结束后发送队列到调度算法
    调度算法 （调度算法传入一个磁盘状态维护实例）
       | <- 调度算法依照算法要求修改读取队列
   块IO缓存层
       | <- 根据当前读取队列头请求磁盘IO
    磁盘驱动
*/

// TODO: 用链表实现以便于移植到嵌入式设备上，以及减少Vec使用的内存碎片和随机插入删除造成的性能浪费

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    AlgoError(Option<String>),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AlgoError(msg) => {
                write!(
                    f,
                    "Algorithm error: {}",
                    match msg {
                        Some(v) => v,
                        None => "No message provided.",
                    }
                )
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Inc,
    Dec,
    Stop,
}

pub trait DiskState {
    fn get_current_position(&self) -> u64;
    fn get_current_direction(&self) -> Direction;
}

pub trait DiskReq {
    fn get_request_address(&self) -> u64;
}

// 磁盘调度算法
pub trait DiskSchedAlgo {
    fn do_algo<'a>(
        disk_state: &dyn DiskState,
        queue: VecDeque<&'a dyn DiskReq>,
    ) -> Result<VecDeque<&'a dyn DiskReq>>;
}
