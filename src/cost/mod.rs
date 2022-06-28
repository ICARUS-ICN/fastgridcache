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

mod last_repeater;

use itertools::Itertools;

use self::last_repeater::LastRepeatIter;

fn cost_sq(start: u64, height: u64, end: u64) -> u64 {
    let w = end - start;
    let h = height;

    if w > 0 {
        h * w * (h + w - 2) / 2
    } else {
        0
    }
}

pub fn get_cost(w: &[u64], h: &[u64]) -> u64 {
    assert!(
        w.len() >= h.len(),
        "Length of w vector ({}) has to be at least the size of vector l({})",
        w.len(),
        h.len()
    );

    let cost_w = calc_cache_costs(w);
    let cost_h = calc_cache_costs(h);

    cost_w + cost_h + cost_full(create_cache_vector(w, h))
}

pub fn cost_full<'a, I>(caches: I) -> u64
where
    I: IntoIterator<Item = &'a u64>,
{
    caches
        .into_iter()
        .tuple_windows()
        .map(|(&w0, &h, &w1)| cost_sq(w0, h, w1))
        .sum()
}

pub fn create_cache_vector<'a>(w: &'a [u64], h: &'a [u64]) -> impl Iterator<Item = &'a u64> + 'a {
    let iter: Box<dyn Iterator<Item = _>> = if w[0] < h[0] {
        Box::new(create_cache_vector_ans(w, h))
    } else {
        Box::new(create_cache_vector_rev(w, h))
    };

    iter
}

fn create_cache_vector_ans<'a, I1, I2>(w: I1, h: I2) -> impl Iterator<Item = &'a u64> + 'a
where
    I1: IntoIterator<Item = &'a u64> + 'a,
    I2: IntoIterator<Item = &'a u64> + 'a,
{
    [&0].into_iter().chain(
        LastRepeatIter::new(h)
            .zip(w)
            .flat_map(|(h1, w1)| [w1, h1].into_iter()),
    )
}

fn create_cache_vector_rev<'a, I1, I2>(w: I1, h: I2) -> impl Iterator<Item = &'a u64> + 'a
where
    I1: IntoIterator<Item = &'a u64> + 'a,
    I2: IntoIterator<Item = &'a u64> + 'a,
{
    [&0].into_iter().chain(
        LastRepeatIter::new(h)
            .zip(w)
            .flat_map(|(h1, w1)| [h1, w1].into_iter()),
    )
}

fn calc_cache_costs(axe_caches: &[u64]) -> u64 {
    if let [.., penultimate, _last] = axe_caches {
        *penultimate
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::get_cost;

    #[test]
    fn square_100_100_3() {
        let w = [36, 73, 100];
        let h = [59, 100];

        assert_eq!(get_cost(&w, &h), 537857);
    }
    #[test]
    fn grid_100_80_4() {
        let h = [44, 74, 100];
        let w = [29, 58, 80];

        assert_eq!(get_cost(&w, &h), 350224);
    }
    #[test]
    fn grid_100_40_3() {
        let w = [25, 50, 75, 100];
        let h = [40];

        assert_eq!(get_cost(&w, &h), 126075);
    }
    #[test]
    fn grid_100_40_4() {
        let w = [32, 54, 77, 100];
        let h = [20, 40];

        assert_eq!(get_cost(&w, &h), 114617);
    }

    #[test]
    fn grid_100_100_1() {
        let w = [50, 100];
        let h = [100];

        assert_eq!(get_cost(&w, &h), 740_050);
    }
}
