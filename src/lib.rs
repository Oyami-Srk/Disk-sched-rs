mod sched;

#[cfg(test)]
mod tests {
    // 我们只模拟算法，并不关心块IO层和磁盘IO驱动的具体实现或模拟。
    use std::collections::VecDeque;

    use rand::{seq::SliceRandom, Rng};

    use crate::sched::{Direction, DiskReq, DiskSchedAlgo, DiskState};
    use crate::sched::{CSCAN, FCFS, SCAN, SSTF};

    struct TestDiskReq {
        addr: u64,
    }

    impl DiskReq for TestDiskReq {
        fn get_request_address(&self) -> u64 {
            self.addr
        }
    }

    struct TestDiskState {
        cur: u64,
        direction: Direction,
    }

    impl DiskState for TestDiskState {
        fn get_current_position(&self) -> u64 {
            self.cur
        }

        fn get_current_direction(&self) -> Direction {
            self.direction
        }
    }

    fn generate_random_req(min: u64, max: u64, len: usize) -> VecDeque<TestDiskReq> {
        let mut rnd = rand::thread_rng();
        let mut v = (min..=max).collect::<Vec<u64>>();
        v.shuffle(&mut rnd);
        VecDeque::from_iter(
            v.as_slice()[0..len]
                .iter()
                .map(|i| TestDiskReq { addr: *i }),
        )
    }

    fn calc_ave_seek_length(cur: u64, scan_queue: &VecDeque<&dyn DiskReq>) -> f64 {
        let queue = {
            let mut t = Vec::new();
            t.push(cur);
            t.extend(scan_queue.iter().map(|v| v.get_request_address()));
            t
        }
        .windows(2)
        .map(|w| {
            if w[0] > w[1] {
                w[0] - w[1]
            } else {
                w[1] - w[0]
            }
        })
        .collect::<Vec<u64>>();
        queue.iter().sum::<u64>() as f64 / queue.len() as f64
    }

    fn print_queue(req_queue: &VecDeque<&dyn DiskReq>) {
        req_queue
            .iter()
            .for_each(|req| print!("{}, ", req.get_request_address()));
        println!();
    }

    fn do_test<T: DiskSchedAlgo>(req_queue: VecDeque<&dyn DiskReq>, state: &dyn DiskState) {
        let raw_len = req_queue.len();
        let scan_queue = T::do_algo(state, req_queue).expect("Algorithm failed.");
        print!("\t算法给出的扫描队列：");
        print_queue(&scan_queue);
        assert_eq!(scan_queue.len(), raw_len);
        println!(
            "\t平均寻道长度：{:.2}",
            calc_ave_seek_length(state.get_current_position(), &scan_queue)
        );
    }

    #[test]
    fn test() {
        println!("小规模数据测试（扇区号1~512，32个请求）：");

        let raw_req = generate_random_req(1, 512, 32);
        let cur = rand::thread_rng().gen_range(1..=512);
        println!("\t随机初始磁头位置：{}", cur);
        let state = TestDiskState {
            cur,
            direction: Direction::Stop,
        };

        print!("\t随机磁盘访问队列：");
        let reqs = VecDeque::<&dyn DiskReq>::from_iter(raw_req.iter().map(|v| v as &dyn DiskReq));
        print_queue(&reqs);

        println!("\n先来先服务算法：");
        do_test::<FCFS>(VecDeque::from_iter(reqs.iter().cloned()), &state);
        println!("\n\t最短寻道时间优先算法：");
        do_test::<SSTF>(VecDeque::from_iter(reqs.iter().cloned()), &state);
        println!("\n\t扫描算法：");
        do_test::<SCAN>(VecDeque::from_iter(reqs.iter().cloned()), &state);
        println!("\n\t循环扫描算法：");
        do_test::<CSCAN>(VecDeque::from_iter(reqs.iter().cloned()), &state);

        println!("\n大规模数据测试：（扇区号1~65535，8192个请求，运行32轮，求平均值）");
        let total = (0..32)
            .map(|_| {
                let raw_req = generate_random_req(1, 65535, 8192);
                let reqs =
                    VecDeque::<&dyn DiskReq>::from_iter(raw_req.iter().map(|v| v as &dyn DiskReq));
                let cur = rand::thread_rng().gen_range(1..=65535);
                let state = TestDiskState {
                    cur,
                    direction: Direction::Stop,
                };

                macro_rules! do_calc {
                    ($ty:ty) => {
                        calc_ave_seek_length(
                            state.cur,
                            &<$ty>::do_algo(&state, VecDeque::from_iter(reqs.iter().cloned()))
                                .expect("Algo failed"),
                        )
                    };
                }

                [
                    do_calc!(FCFS),
                    do_calc!(SSTF),
                    do_calc!(SCAN),
                    do_calc!(CSCAN),
                ]
            })
            .fold([0.0, 0.0, 0.0, 0.0], |acc, x| {
                [acc[0] + x[0], acc[1] + x[1], acc[2] + x[2], acc[3] + x[3]]
            });
        let avg = total.map(|v| v / 32.0);
        println!("\t先来先服务算法的平均寻道长度：{:.2}", avg[0]);
        println!("\t先来先服务算法的平均寻道长度：{:.2}", avg[1]);
        println!("\t先来先服务算法的平均寻道长度：{:.2}", avg[2]);
        println!("\t先来先服务算法的平均寻道长度：{:.2}", avg[3]);
    }
}
