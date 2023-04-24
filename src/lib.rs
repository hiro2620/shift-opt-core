use std::collections::HashSet;
pub mod staff_table;
mod maximum_bipartite_matching;

pub use staff_table::AssignableStaffTable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Task(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Time(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Staff(usize);

#[derive(Debug, Clone)]
pub struct StaffInput {
    idx: usize,
    assignable_task: HashSet<Task>,
    assignable_time: HashSet<Time>
}