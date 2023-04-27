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

    pub fn new_from_table(task_cnt:usize, time_cnt:usize, staff_cnt:usize, table:&Vec<Vec<Vec<usize>>>) -> Self {
        let mut table_staff = vec![vec![vec![];task_cnt];time_cnt];
        for i in 0..time_cnt {
            for j in 0..task_cnt {
                table_staff[i][j] = table[i][j].iter().map(|&v| { Staff(v) }).collect::<Vec<_>>();
            }
        }
        Self {
            task_cnt, time_cnt, staff_cnt, table:table_staff,
        }
    }

    fn solve_inner(&self) -> StaffTable {
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

    #[doc = "res[i][j]: i番目の仕事,j番目の時間に入る人(入力に対して転置されていることに注意)"]
    pub fn solve(&self) -> Vec<Vec<Option<usize>>> {
        let r = self.solve_inner().table;

        let mut result = vec![vec![None;self.time_cnt];self.task_cnt];

        for i in 0..self.task_cnt {
            for j in 0..self.time_cnt {
                let v = r[j][i];
                if v.is_some() {
                    result[i][j] = Some(v.unwrap().0);
                } else {
                    result[i][j] = None;
                }
            }
        };

        result
    }

    fn solve_each_time(&self, time: usize) -> Vec<Option<Staff>> {

        let shift_size = 2; // 添字をいくつずつずらすか
        let shift_span = 2; // 添字を何回置きにずらすか
        let m = self.staff_cnt;

        let current_shift_size = (time/shift_span) * shift_size % m;

        let time_record = &self.table[time];

        let time_record_shift = time_record.iter().map(|v| {
            let mut vc = v.clone();
            vc.rotate_left(current_shift_size);
            vc
        }).collect::<Vec<_>>();

        let time_res = assign_tasks(&time_record_shift, self.staff_cnt);

        time_res.iter().map(|v| {
            match v {
                Some(v) => Some(Staff((v.0+current_shift_size)%m)),
                None => None
            }
        }).collect::<Vec<_>>()
    }
}