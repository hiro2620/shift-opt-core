use crate::{Staff, StaffInput, Task, Time};
use crate::maximum_bipartite_matching::assign_tasks;

#[doc = "StaffTable[i][j]: 時間iでタスクjに割り当てられたスタッフ"]
pub struct StaffTable {
    pub task_cnt: usize,
    pub time_cnt: usize,
    pub staff_cnt: usize,
    pub table: Vec<Vec<Option<Staff>>>
}

#[doc = "StaffTable[i][j]: 時間iでタスクjに割り当て可能なスタッフ"]
pub struct AssignableStaffTable {
    task_cnt: usize,
    time_cnt: usize,
    staff_cnt: usize,
    table: Vec<Vec<Vec<Staff>>>
}

impl AssignableStaffTable {
    pub fn new(task_cnt:usize, time_cnt:usize, staff:&Vec<StaffInput>) -> Self {
        let mut table = vec![vec![vec![];task_cnt];time_cnt];

        let staff_cnt = staff.len();

        for s in staff {
            for ti in 0..time_cnt {
                for ta in 0..task_cnt {
                    if s.assignable_time.contains(&Time(ti)) && s.assignable_task.contains(&Task(ta)) {
                        table[ti][ta].push(Staff(s.idx));
                    }
                }
            }
        };

        Self {
            task_cnt, time_cnt, table, staff_cnt
        }
    }

    pub fn solve(&self) -> StaffTable {
        let mut res = vec![];

        for ti in 0..self.time_cnt {
            let r = self.solve_each_time(ti);
            res.push(r);
        }

        StaffTable {
            task_cnt: self.task_cnt,
            time_cnt: self.time_cnt,
            staff_cnt: self.staff_cnt,
            table: res,
        }
    }

    fn solve_each_time(&self, time: usize) -> Vec<Option<Staff>> {
        let time_record = &self.table[time];
        let time_res = assign_tasks(time_record, self.staff_cnt);

        time_res
    }
}