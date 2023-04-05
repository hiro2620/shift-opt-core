use crate::Staff;
mod dinic_algo;

#[doc = "TaskStaffInput[i]: i番目のTaskを割り当て可能なStaffのVec"]
pub struct TaskStaffInput(Vec<Vec<Staff>>);