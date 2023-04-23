use crate::Staff;
mod dinic_algo;
mod assign_tasks;

pub use assign_tasks::assign_tasks;

#[doc = "TaskStaffInput[i]: i番目のTaskを割り当て可能なStaffのVec"]
pub type TaskStaffInput<'a> = &'a Vec<Vec<Staff>>;

#[doc = "TaskStaffInput[i]: i番目のTaskを割り当て可能なStaff"]
pub type  TaskStaffResult = Vec<Option<Staff>>;