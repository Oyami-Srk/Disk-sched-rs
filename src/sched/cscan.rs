use std::collections::VecDeque;

use super::{Direction, DiskReq, DiskSchedAlgo, DiskState, Result, Utils};

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
        let closet_idx = Utils::found_closet_req_index(cur, &queue);
        let direction = match disk_state.get_current_direction() {
            Direction::Inc => Direction::Inc,
            Direction::Dec => Direction::Dec,
            Direction::Stop => {
                if cur >= queue[closet_idx].get_request_address() {
                    Direction::Dec
                } else {
                    Direction::Inc
                }
            }
        };
        if let Direction::Dec = direction {
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
