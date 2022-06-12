use std::collections::VecDeque;

use super::{Direction, DiskReq, DiskSchedAlgo, DiskState, Error, Result, Utils};

pub struct SSTF;

// 短寻道时间优先构建一个贪心选择队列，每次选择与现在距离最近的扇区
/* 算法首先对待访问的扇区进行排序，然后确定距离当前磁头所在的最近的扇区
 * A B C
 *   ^ 然后将B放入返回队列，对比A和C，确定接下来的移动哪个
 */
impl DiskSchedAlgo for SSTF {
    fn do_algo<'a>(
        disk_state: &dyn DiskState,
        mut queue: VecDeque<&'a dyn DiskReq>,
    ) -> Result<VecDeque<&'a dyn DiskReq>> {
        let cur = disk_state.get_current_position();
        // Sort queue by address order
        queue
            .make_contiguous()
            .sort_by(|v1, v2| v1.get_request_address().cmp(&v2.get_request_address()));
        let mut result_queue = VecDeque::new();
        // fount cloest addr with current position
        let mut closet_idx = Utils::found_closet_req_index(cur, &queue);
        // initial direction of current pos to closed pos
        // direction is used when the seek time is identical
        // in that context, we chose the same direction.
        let mut move_direction = if cur > queue[closet_idx].get_request_address() {
            Direction::Dec
        } else {
            Direction::Inc
        };
        loop {
            let cur_req = queue.remove(closet_idx).unwrap();
            if queue.len() == 0 {
                result_queue.push_back(cur_req);
                break;
            }
            if closet_idx == queue.len() {
                closet_idx -= 1;
                move_direction = Direction::Dec;
            } else if closet_idx != 0 {
                let left = (cur_req.get_request_address() as i64
                    - queue[closet_idx - 1].get_request_address() as i64)
                    .abs();
                let right = (cur_req.get_request_address() as i64
                    - queue[closet_idx].get_request_address() as i64)
                    .abs();
                if right < left {
                    move_direction = Direction::Inc;
                } else if right == left {
                    // When seek time is same, we chose the same direction.
                    closet_idx = match move_direction {
                        Direction::Dec => closet_idx - 1,
                        Direction::Inc => closet_idx,
                        Direction::Stop => {
                            return Err(Error::AlgoError(Some(
                                "Direction cannot be stop.".to_string(),
                            )));
                        }
                    };
                } else {
                    closet_idx -= 1;
                    move_direction = Direction::Dec;
                }
            }
            result_queue.push_back(cur_req);
        }
        Ok(result_queue)
    }
}
