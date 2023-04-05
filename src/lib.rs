use std::collections::HashSet;

mod maximum_bipartite_matching;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Task(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Staff(usize);

#[derive(Debug, Clone)]
pub struct StaffInput {
    idx: usize,
    assignable_task: HashSet<Task>,
    assignable_time: HashSet<Time>
}

#[doc = "StaffTable[i][j]: 時間iでタスクjに割り当てられたスタッフ"]
pub struct StaffTable {
    task_cnt: usize,
    time_cnt: usize,
    table: Vec<Vec<Option<Staff>>>
}

#[doc = "StaffTable[i][j]: 時間iでタスクjに割り当て可能なスタッフ"]
pub struct AssignableStaffTable {
    task_cnt: usize,
    time_cnt: usize,
    table: Vec<Vec<Vec<Staff>>>
}

impl AssignableStaffTable {
    pub fn new(task_cnt:usize, time_cnt:usize, staff:&Vec<StaffInput>) -> Self {
        let mut table = vec![vec![vec![];task_cnt];time_cnt];

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
            task_cnt, time_cnt, table
        }
    }

    pub fn solve(&self) -> StaffTable {
        let mut res = vec![vec![]];

        for ti in 0..self.time_cnt {
            self.solve_each_time(&mut res, ti);
        }

        StaffTable {
            task_cnt: self.task_cnt,
            time_cnt: self.time_cnt,
            table: res
        }
    }

    fn solve_each_time(&self, res: &mut Vec<Vec<Option<Staff>>>,time: usize) {
        let time_record = &self.table[time];


    }
}