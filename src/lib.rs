// Copyright 2020 Xavier Gillard
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


//! This crate generates graph using my take on the Erdos-Renyi model.
//! Its point is to provide a *simple* and reasonably fast way to generate
//! random graphs as input for my experiments.

extern crate rand;

use rand::thread_rng;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use std::collections::HashMap;

/// The configuration of an Erdos-Renyi G(n, p) model.
#[derive(Debug, Clone, Copy)]
pub struct ErModel {
    /// Number of vertices in the generated graphs
    n: usize,
    /// Likelihood of any edge to be generated
    p: f64,
    /// this flag is true
    digraph: bool,
    /// Allow self loops
    self_loops: bool
}
impl ErModel {
    pub fn new(n: usize, p: f64) -> Self {
        ErModel {n, p, self_loops: false, digraph: false}
    }
    pub fn digraph(self) -> Self {
        ErModel{n: self.n, p: self.p, self_loops: self.self_loops, digraph: true}
    }
    pub fn with_self_loops(self) -> Self {
        ErModel{n: self.n, p: self.p, self_loops: true, digraph: self.digraph}
    }
    /// returns a new generator for the given model
    pub fn generator(self) -> ErGenerator {
        ErGenerator::new(self)
    }

    /// returns the number of edges if the graph were full mesh
    fn nb_possible_edges(self) -> u128 {
        let sources = self.n as u128;
        let dests   = if self.self_loops { self.n } else { self.n - 1 } as u128;

        if self.digraph {
            sources * dests
        } else {
            (sources * dests) / 2
        }
    }
    /// returns the number of edges that should be sampled so that each of the
    /// candidate edges has a likelihood of p.
    fn nb_edges_to_pick(self) -> usize {
        (self.p * self.nb_possible_edges() as f64).round() as usize
    }
}
/// A vertex is basically just a typesafe integer id of the vertex
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vertex {
    /// The vertex identifier
    id: usize
}

/// An edge connects two vertices
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Edge {
    /// The source end of the edge
    src   : Vertex,
    /// The target end of the edge
    dst   : Vertex,
}
impl Edge {
    /// Returns true iff this edge is self looping
    pub fn is_self_loop(self) -> bool {
        self.src == self.dst
    }
    pub fn rev(self) -> Self {
        Edge {src: self.dst, dst: self.src}
    }
}

/// A graph as can be random generated
pub struct Graph {
    model: ErModel,
    n    : usize,
    list : HashMap<Edge, isize>
}

impl Graph {
    pub fn pluck_random_weights(&mut self, from: &[isize]) {
        let mut rng = thread_rng();
        let dist= Uniform::new(0, from.len());

        for (_e, w) in self.list.iter_mut() {
            *w = from[dist.sample(&mut rng)];
        }
    }
    pub fn to_dimacs(&self) -> String {
        let mut out = vec![];

        let gtype = if self.model.digraph    { "digraph" } else {"graph"};
        let loops = if self.model.self_loops { "" }        else { " NOT"};
        out.push(format!("c Pseudo-random Erdos-Renyi {} G({}, {})", gtype, self.model.n, self.model.p));
        out.push(format!("c it was generated to{} allow self loops", loops));
        out.push(format!("c This graph has {} vertices and {} edges", self.n, self.list.len()));

        out.push(format!("{} {}", self.n, self.list.len()));

        for (edge, w) in self.list.iter() {
            out.push(format!("{} {} {}", edge.src.id, edge.dst.id, w));
        }

        out.join("\n")
    }

    pub fn to_dot(&self) -> String {
        let mut out = vec![];

        let gtype     = if self.model.digraph { "digraph" } else {"graph"};
        let connector = if self.model.digraph { "->" }      else { "--" };
        out.push(format!("{} g {{", gtype));
        for v in 0..self.n {
            out.push(format!("  {};", v));
        }
        for (edge, w) in self.list.iter() {
            out.push(format!("  {} {} {} [label={}];", edge.src.id, connector, edge.dst.id, w));
        }
        out.push("}".to_owned());

        out.join("\n")
    }
}

/// The graph generator using ER model
#[derive(Debug)]
pub struct ErGenerator {
    /// The er model
    model: ErModel,
    /// The random number generator
    rng  : ThreadRng,
    /// Uniform distribution to pick numbers from
    dist : Uniform<u128>
}

impl ErGenerator {
    fn new(model: ErModel) -> ErGenerator {
        ErGenerator {
            model,
            rng  : thread_rng(),
            dist : Uniform::new(0, model.n as u128 * model.n as u128)
        }
    }

    fn next_edge(&mut self) -> Edge {
        let dist = &mut self.dist;
        let rng = &mut self.rng;

        let number = dist.sample(rng);

        let src = (number / self.model.n as u128) as usize;
        let dst = (number % self.model.n as u128) as usize;

        // Typically, graph formats dont like numberings to start at zero,
        // I dont know why because it is sooooooo convenient.
        Edge { src: Vertex{id: src + 1}, dst: Vertex{id: dst + 1} }
    }

    pub fn gen(&mut self) -> Graph {
        let mut g = Graph{model: self.model, n: self.model.n, list: Default::default()};

        let nb_edges = self.model.nb_edges_to_pick();
        g.list.reserve(nb_edges);

        while g.list.len() < nb_edges {
            let edge = self.next_edge();

            if edge.is_self_loop() && !self.model.self_loops {
                continue;
            }

            if g.list.contains_key(&edge) || g.list.contains_key(&edge.rev()) {
                continue;
            }

            g.list.insert(edge, 1);
        }

        g
    }
}
impl Iterator for ErGenerator {
    type Item = Graph;
    fn next(&mut self) -> Option<Graph> {
        Some(self.gen())
    }
}
