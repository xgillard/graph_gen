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

use graph_gen::{ErModel, Graph, Max2SatGraph, Generatable};
use structopt::StructOpt;
use crate::Output::Dimacs;
use std::str::FromStr;

/// Convenience tool to generate pseudo random graphs.
#[derive(StructOpt)]
struct Args {
    /// The number of vertices in the generated graph
    #[structopt(name="nb_vertices", short, long)]
    n: usize,
    /// The likelihood of any edge to be picked
    #[structopt(name="probability", short, long)]
    p: f64,
    /// If set, self loops are allowed in the generated graph
    #[structopt(name="loops", short, long)]
    loops: bool,
    /// If set, the generated graph will be a digraph
    #[structopt(name="digraph", short, long)]
    digraph: bool,
    /// If set, the generated graph will be a max2sat instance
    #[structopt(name="max2sat", short, long)]
    max2sat: bool,
    /// The output language (defaults to dimacs)
    #[structopt(name="output", short, long)]
    output : Option<Output>,
    /// Optional weight candidates
    #[structopt(name="weights", short, long)]
    weights: Option<Vec<isize>>
}
enum Output {
    Dimacs, GraphViz
}
impl Default for Output {
    fn default() -> Self {
        Dimacs
    }
}
impl FromStr for Output {
    type Err = String;

    fn from_str(txt: &str) -> Result<Output, String> {
        if &txt.to_lowercase() == "dimacs" {
            return Ok(Output::Dimacs);
        }
        if &txt.to_lowercase() == "graphviz" {
            return Ok(Output::GraphViz);
        }
        if &txt.to_lowercase() == "dot" {
            return Ok(Output::GraphViz);
        }

        Err(txt.to_owned())
    }
}

impl Args {
    fn graph(&self) -> Graph {
        let n = if self.max2sat { 2 * self.n } else { self.n };

        let mut model = ErModel::new(n, self.p);

        if self.digraph {
            model = model.digraph();
        }

        if self.loops {
            model = model.with_self_loops();
        }

        let mut graph = model.generator().gen();

        if let Some(weights) = self.weights.as_ref() {
            graph.pluck_random_weights(weights);
        }

        graph
    }

    fn wcnf(&self, g: Graph) -> Max2SatGraph {
        Max2SatGraph::new(g)
    }

    fn generatable(&self) -> Generatable {
        let graph = self.graph();

        if self.max2sat {
            Generatable::GenSat   {s : self.wcnf(graph)}
        } else {
            Generatable::GenGraph {g : graph}
        }
    }

    fn output(&self, g: &Generatable) -> String {
        match &self.output {
            None => g.to_dimacs(),
            Some(o) => match o {
                Output::Dimacs   => g.to_dimacs(),
                Output::GraphViz => g.to_dot()
            }
        }
    }
}

fn main() {
    let args = Args::from_args();
    let graph= args.generatable();
    let out  = args.output(&graph);

    println!("{}", out);
}
