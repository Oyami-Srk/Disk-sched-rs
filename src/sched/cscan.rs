use std::collections::VecDeque;

use crate::sched::DiskReq;

use super::{DiskSchedAlgo, DiskState, Result};

pub struct CSCAN;

// 循环扫描算法是对电梯算法的改进，为了平衡等待时间，所以到头了就直接返回另一头的最开始而不是沿途扫描
impl DiskSchedAlgo for CSCAN {
    fn do_algo<'a>(
        disk_state: &dyn DiskState,
        mut queue: VecDeque<&'a dyn DiskReq>,
    ) -> Result<VecDeque<&'a dyn DiskReq>> {
        let cur = disk_state.get_current_position();
        queue
            .make_contiguous()
            .sort_by(|v1, v2| v1.get_request_address().cmp(&v2.get_request_address()));
        let mut result_queue = VecDeque::new();
        let closet_idx = if cur <= queue[0].get_request_address() {
            0
        } else if cur >= queue[queue.len() - 1].get_request_address() {
            queue.len() - 1
        } else {
            queue
                .iter()
                .enumerate()
                .min_by_key(|(_, v)| (v.get_request_address() as i64 - cur as i64).abs())
                .map(|(index, _)| index)
                .unwrap()
        };
        if cur >= queue[closet_idx].get_request_address() {
            // go downward then upward
            for idx in (0..=closet_idx).rev() {
                result_queue.push_back(queue.remove(idx).unwrap());
            }
            while queue.len() != 0 {
                result_queue.push_back(queue.pop_back().unwrap());
            }
        } else {
            // go upward then downward
            while closet_idx < queue.len() {
                result_queue.push_back(queue.remove(closet_idx).unwrap());
            }
            while queue.len() != 0 {
                result_queue.push_back(queue.pop_front().unwrap());
            }
        }
        Ok(result_queue)
    }
}
