use super::{TaskStaffInput, TaskStaffResult};
use super::dinic_algo::{Dinic, Edge};
use crate::Staff;

pub fn assign_tasks(input: TaskStaffInput, staff_cnt:usize) -> TaskStaffResult {
  let task_cnt = input.len();

  let n = task_cnt+staff_cnt+2;

  // 0:始点, n-1:終点
  // 1~task_cnt: task
  // task_cnt+1~task_cnt+staff_cnt: staff
  let mut dinic = Dinic::new(n);

  for i in 1..=staff_cnt {
    dinic.add_edge(&Edge { from: 0, to: i, cap: 1 });
  }

    for i in task_cnt+1..=task_cnt+staff_cnt {
    dinic.add_edge(&Edge { from: i, to: n-1, cap: 1 });
  }

  for (i_task, staffs) in input.iter().enumerate() {
    for staff in staffs {
      let i_staff = staff.0;
      dinic.add_edge(&Edge{
        from: i_task+1,
        to: task_cnt+i_staff+1,
        cap: 1,
      })
    }
  }

  let res_dinic = dinic.max_flow(0, n-1);

  let mut res:TaskStaffResult = vec![None;task_cnt];

  for v in res_dinic.flows.iter() {
    if v.cap == 0 {
      continue;
    }
    if 1<= v.from && v.from <= task_cnt {
      res[v.from-1] = Some(Staff(v.to-1-task_cnt));
    }
  };

  res
}

#[cfg(test)]
mod test {
  use crate::Staff;

use super::assign_tasks;

  #[test]
  fn test_assign_tasks() {
    let n = 6;

    let input = vec![
      vec![Staff(0)],
      vec![Staff(1),Staff(3),Staff(5)],
      vec![Staff(5)],
      vec![Staff(2)],
      vec![Staff(2)],
      vec![Staff(5)],
    ];

    let res = assign_tasks(&input, n);
    dbg!(res);
  }
}