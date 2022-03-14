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
use fastgridcache::get_cache_locations;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the network grid
    #[clap(short, long)]
    width: u64,

    /// Height of the network grid
    #[clap(short, long)]
    height: u64,

    /// Number of caches
    #[clap(short, long, default_value_t = 0)]
    ncaches: u16,

    #[clap(short = 'c', long)]
    hide_cost: bool,

    #[clap(short, long)]
    show_caches: bool,
}
fn main() {
    let args = Args::parse();

    if args.width < args.height {
        eprintln!("Grid cannot be taller than wider.");
        exit(exitcode::DATAERR);
    }

    let (best_cost, best_w, best_h) = match (0..=args.ncaches / 2)
        .into_par_iter()
        .map(|hcaches_n| {
            let sol = get_cache_locations(
                args.width,
                args.height,
                (args.ncaches - hcaches_n).into(),
                hcaches_n.into(),
            );

            (
                sol.cost(),
                sol.horizontal_caches().to_owned(),
                sol.vertical_caches().to_owned(),
            )
        })
        .min_by(|(cost1, _w1, _h1), (cost2, _w2, _h2)| cost1.cmp(cost2))
    {
        Some((cost, w, h)) => (cost, w, h),
        _ => (u64::MAX, vec![], vec![]),
    };

    if !args.hide_cost {
        println!("{best_cost}");
    }
    if args.show_caches {
        println!("{:?}×{:?}", best_w, best_h);
    }

    exit(exitcode::OK);
}
