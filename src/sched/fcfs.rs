use std::collections::VecDeque;

use crate::sched::DiskReq;

use super::{DiskSchedAlgo, DiskState, Result};

pub struct FCFS;

// 先来先服务算法不对等待队列进行任何重排，先进入队列的先出队列被服务
impl DiskSchedAlgo for FCFS {
    fn do_algo<'a>(
        disk_state: &dyn DiskState,
        queue: VecDeque<&'a dyn DiskReq>,
    ) -> Result<VecDeque<&'a dyn DiskReq>> {
        let _ = disk_state;
        Ok(queue)
    }
}
