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

pub fn get_cost<I1, I2, Iter1, Iter2>(w: I1, h: I2) -> u64
where
    I1: IntoIterator<IntoIter = Iter1, Item = u64>,
    I2: IntoIterator<IntoIter = Iter2, Item = u64>,
    Iter1: Clone + DoubleEndedIterator<Item = u64> + ExactSizeIterator,
    Iter2: Clone + DoubleEndedIterator<Item = u64> + ExactSizeIterator,
{
    let w_iter = w.into_iter();
    let h_iter = h.into_iter();

    assert!(
        w_iter.len() >= h_iter.len(),
        "Length of w vector ({}) has to be at least the size of vector l({})",
        w_iter.len(),
        h_iter.len()
    );

    let cost_w = calc_cache_costs(w_iter.clone());
    let cost_h = calc_cache_costs(h_iter.clone());

    cost_w + cost_h + cost_full(create_cache_vector(w_iter, h_iter))
}

pub fn cost_full<I>(caches: I) -> u64
where
    I: IntoIterator<Item = u64>,
{
    caches
        .into_iter()
        .tuple_windows()
        .map(|(w0, h, w1)| cost_sq(w0, h, w1))
        .sum()
}

pub fn create_cache_vector<'a, I1, I2>(w: I1, h: I2) -> impl Iterator<Item = u64> + 'a
where
    I1: IntoIterator<Item = u64> + 'a,
    I2: IntoIterator<Item = u64> + 'a,
{
    let mut w_iter = w.into_iter();
    let mut h_iter = h.into_iter();

    let w1 = w_iter.next().unwrap();
    let h1 = h_iter.next().unwrap();

    let iter: Box<dyn Iterator<Item = _>> = if w1 < h1 {
        Box::new(create_cache_vector_ans(
            [w1].into_iter().chain(w_iter),
            [h1].into_iter().chain(h_iter),
        ))
    } else {
        Box::new(create_cache_vector_rev(
            [w1].into_iter().chain(w_iter),
            [h1].into_iter().chain(h_iter),
        ))
    };

    iter
}

fn create_cache_vector_ans<'a, I1, I2>(w: I1, h: I2) -> impl Iterator<Item = u64> + 'a
where
    I1: IntoIterator<Item = u64> + 'a,
    I2: IntoIterator<Item = u64> + 'a,
{
    [0].into_iter().chain(
        LastRepeatIter::new(h)
            .zip(w)
            .flat_map(|(h1, w1)| [w1, h1].into_iter()),
    )
}

fn create_cache_vector_rev<'a, I1, I2>(w: I1, h: I2) -> impl Iterator<Item = u64> + 'a
where
    I1: IntoIterator<Item = u64> + 'a,
    I2: IntoIterator<Item = u64> + 'a,
{
    [0].into_iter().chain(
        LastRepeatIter::new(h)
            .zip(w)
            .flat_map(|(h1, w1)| [h1, w1].into_iter()),
    )
}

fn calc_cache_costs<I, IterType>(axe_caches: I) -> u64
where
    I: IntoIterator<IntoIter = IterType>,
    IterType: DoubleEndedIterator<Item = u64>,
{
    axe_caches.into_iter().nth_back(1).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use crate::get_cost;

    #[test]
    fn square_100_100_3() {
        let w = [36, 73, 100];
        let h = [59, 100];

        assert_eq!(get_cost(w, h), 537857);
    }
    #[test]
    fn grid_100_80_4() {
        let h = [44, 74, 100];
        let w = [29, 58, 80];

        assert_eq!(get_cost(w, h), 350224);
    }
    #[test]
    fn grid_100_40_3() {
        let w = [25, 50, 75, 100];
        let h = [40];

        assert_eq!(get_cost(w, h), 126075);
    }
    #[test]
    fn grid_100_40_4() {
        let w = [32, 54, 77, 100];
        let h = [20, 40];

        assert_eq!(get_cost(w, h), 114617);
    }
}
