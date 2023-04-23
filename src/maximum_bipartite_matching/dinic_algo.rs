use std::{collections::VecDeque};
const INF:usize = 1<<60;

#[doc = "Dinicの入出力に用いる辺"]
#[derive(Debug)]
pub struct Edge {
  pub from: usize,
  pub to: usize,
  pub cap: usize,
}

#[doc = "処理に用いる辺"]
#[derive(Debug, Clone, Copy)]
struct EdgeInner {
  to: usize,
  cap: usize,
  rev_idx: usize,
  is_rev: bool,
}

#[doc = "max_flowの戻り値"]
#[derive(Debug)]
pub struct DinicMaxFlowResult {
  pub flow_total: usize,
  pub flows: Vec<Edge>,
}

// 参考1: https://ikatakos.com/pot/programming_algorithm/graph_theory/maximum_flow#%E5%AE%9F%E8%A3%85
// 参考2: https://ei1333.github.io/luzhiled/snippets/graph/dinic.html
#[doc = "グラフの最大フローを求める"]
#[derive(Debug)]
pub struct Dinic {
  n: usize,
  g: Vec<Vec<EdgeInner>>,
  level: Vec<Option<usize>>,
  progress: Vec<usize>,
}

impl Dinic {

  pub fn new(n: usize) -> Self {
    let g = vec![vec![];n];
    Dinic { n, g, level: vec![None;n], progress: vec![0;n] }
  }

  pub fn add_edge(&mut self, e: &Edge) {
    let rev_idx = self.g[e.to].len();
    self.g[e.from].push(EdgeInner { to: e.to, cap: e.cap, rev_idx, is_rev:false });

    let rev_idx = self.g[e.from].len() - 1;
    self.g[e.to].push(EdgeInner { to: e.from, cap: 0, rev_idx, is_rev:true });
  }

  #[doc = "sからの最短距離(level)をbfsで計算"]
  fn bfs(&mut self, s:usize, t:usize) -> bool {
    self.level = vec![None;self.n];
    self.level[s] = Some(0);

    let mut queue = VecDeque::from([s]);

    while let Some(u) = queue.pop_front() {
      for e in self.g[u].iter() {
        if e.cap > 0 && self.level[e.to].is_none() {
          self.level[e.to] = Some(self.level[u].unwrap() + 1);
          queue.push_back(e.to);
        }
      }
    };

    self.level[t].is_some()
  }

  #[doc = "増加バスをdfsで探す"]
  fn dfs(&mut self, v:usize, t:usize, flow:usize) -> usize {
    if v == t {
      return flow;
    };

    for i in self.progress[v]..self.g[v].len() {
      self.progress[v] = i;
      let e = self.g[v][i];

      if e.cap > 0 && self.level[v] < self.level[e.to] {
        let d = self.dfs(e.to, t, flow.min(e.cap));
        if d > 0 {
          self.g[v][i].cap -= d;
          self.g[e.to][e.rev_idx].cap += d;

          return d;
        }
      }
    };

    0
  }

  pub fn max_flow(&mut self, s:usize, t:usize) -> DinicMaxFlowResult {
    let mut flow = 0;
    while self.bfs(s, t) {
      self.progress = vec![0; self.n];
      let mut f;
      loop {
        f = self.dfs(s, t, INF);
        if f == 0 {
          break;
        }
        flow += f;
      }
    };

    let mut flows = vec![];
    for i in 0..self.n {
      for &e in self.g[i].iter() {
        if e.is_rev {
          continue;
        }

        let rev_e = self.g[e.to][e.rev_idx];
        flows.push(Edge { from: i, to: e.to, cap: rev_e.cap });
      }
    }

    DinicMaxFlowResult { flow_total: flow, flows }
  }

  #[allow(unused)]
  pub fn show_flow(&self) {
    for i in 0..self.n {
      for &e in self.g[i].iter() {
        if e.is_rev {
          continue;
        }

        let rev_e = self.g[e.to][e.rev_idx];
        println!("{} -> {} (flow: {}/{})",i, e.to, rev_e.cap, e.cap+rev_e.cap)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_dinic() {
    // Case #1
    let mut dinic = Dinic::new(3);

    dinic.add_edge(&Edge {from:0,to:1,cap:2});
    dinic.add_edge(&Edge {from:1,to:2,cap:1});
    dinic.add_edge(&Edge {from:0,to:2,cap:3});

    let ans = dinic.max_flow(0, 2);
    dinic.show_flow();
    assert_eq!(ans.flow_total, 4);

    // Case #2
    let edges = [
      Edge {from:0,to:1,cap:2},
      Edge {from:0,to:3,cap:2},
      Edge {from:0,to:4,cap:2},
      Edge {from:1,to:2,cap:4},
      Edge {from:1,to:4,cap:3},
      Edge {from:2,to:5,cap:3},
      Edge {from:3,to:4,cap:1},
      Edge {from:4,to:2,cap:1},
      Edge {from:4,to:5,cap:3},
    ];
    let mut dinic = Dinic::new(6);
    for e in &edges {
      dinic.add_edge(e);
    }

    let ans = dinic.max_flow(0, 5);
    dinic.show_flow();
    assert_eq!(ans.flow_total, 5);
  }
}