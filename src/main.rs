// SPDX-License-Identifier: GPL-3.0-or-later
/*
 *
 * Copyright (c) 2022 Universidade de Vigo
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation;
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Author: Miguel Rodríguez Pérez <miguel@det.uvigo.gal>
 *
 */

use std::process::exit;

use clap::StructOpt;
use fastgridcache::get_cost;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the network grid
    #[clap(short, long)]
    width: u32,

    /// Height of the network grid
    #[clap(short, long)]
    height: u32,

    /// Number of caches
    #[clap(short, long, default_value_t = 0)]
    ncaches: u8,
}
fn main() {
    let args = Args::parse();

    if args.width < args.height {
        eprintln!("Grid cannot be taller than wider.");
        exit(exitcode::DATAERR);
    }
    //Vector: [0, 27, 37, 54, 60, 77, 60, 100, 60]
    //Vector: [0, 25, 34, 50, 56, 50, 78, 50, 100]

    let (best_cost, best_w, best_h) = match (0..=args.ncaches / 2)
        .into_par_iter()
        .map(|hcaches_n| {
            let mut best_partial_cost = u64::MAX;
            let mut best_partial_w = Vec::new();
            let mut best_partial_h = Vec::new();

            let wcaches_n = args.ncaches - hcaches_n;

            for w_vector in (1..args.width).combinations(wcaches_n.into()) {
                let w_vector_full: Vec<_> = w_vector.iter().chain(&[args.width]).copied().collect();
                for h_vector in (1..args.height).combinations(hcaches_n.into()) {
                    let h_vector_full: Vec<_> =
                        h_vector.iter().chain(&[args.height]).copied().collect();

                    let cost = get_cost(w_vector_full.clone(), h_vector_full);

                    if cost < best_partial_cost {
                        best_partial_cost = cost;
                        best_partial_w = w_vector.clone();
                        best_partial_h = h_vector.clone();
                    }
                }
            }
            (best_partial_cost, best_partial_w, best_partial_h)
        })
        .min_by(|(cost1, _w1, _h1), (cost2, _w2, _h2)| cost1.cmp(cost2))
    {
        Some((cost, w, h)) => (cost, w, h),
        _ => (u64::MAX, Vec::new(), Vec::new()),
    };

    println!(
        "Best combination is {:?}×{:?} with cost {best_cost}",
        best_w, best_h
    );

    exit(exitcode::OK);
}
