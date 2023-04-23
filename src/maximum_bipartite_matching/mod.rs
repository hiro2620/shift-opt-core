use crate::Staff;
mod dinic_algo;
mod assign_tasks;

#[doc = "TaskStaffInput[i]: i番目のTaskを割り当て可能なStaffのVec"]
pub type TaskStaffInput = Vec<Vec<Staff>>;

#[doc = "TaskStaffInput[i]: i番目のTaskを割り当て可能なStaff"]
pub type  TaskStaffResult = Vec<Option<Staff>>;