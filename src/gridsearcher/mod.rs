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

mod horizontal;
mod vertical;

use std::cmp::min;

use crate::{cost::cost_full, filtered_slice::FilteredSlice};

use self::{horizontal::SearcherFirstHorizontal, vertical::SearcherFirstVertical};

#[derive(Debug)]
struct Problem {
    nhoriz: usize,
    nvert: usize,
    width: u64,
    filtered: FilteredSlice<Vec<u64>, usize>,
}

impl Problem {
    /// Get a mutable reference to the problem's filtered.
    fn filtered_mut(&mut self) -> &mut FilteredSlice<Vec<u64>, usize> {
        &mut self.filtered
    }

    /// Get a reference to the problem's filtered.
    fn filtered(&self) -> &FilteredSlice<Vec<u64>, usize> {
        &self.filtered
    }
}

trait Searcher {
    fn width(&self) -> u64;

    /// Get a mutable reference to the problem's filtered.
    fn filtered_mut(&mut self) -> &mut FilteredSlice<Vec<u64>, usize>;

    /// Get a reference to the problem's filtered.
    fn filtered(&self) -> &FilteredSlice<Vec<u64>, usize>;

    fn try_advance_index(&mut self, index: usize) -> bool {
        if self.filtered()[index] >= self.width()
            || index + 1 < self.filtered().len()
                && self.filtered()[index] >= self.filtered()[index + 1]
        {
            return false;
        }

        let prev = self.filtered()[index];
        let cost = cost_full(self.filtered().get_raw_ref());
        self.filtered_mut()[index] += 1;
        let new_cost = cost_full(self.filtered().get_raw_ref());
        self.filtered_mut()[index] = prev;

        new_cost <= cost
    }

    fn advance_index(&mut self, index: usize) {
        self.filtered_mut()[index] += 1;
    }

    fn get_cost(&self) -> u64 {
        let cache_costs: u64 = self.horizontal_caches().last().unwrap_or(0)
            + self.vertical_caches().last().unwrap_or(0);

        cache_costs + cost_full(self.filtered().get_raw_ref())
    }

    fn vertical_caches(&self) -> Box<dyn Iterator<Item = u64> + '_>;

    fn horizontal_caches(&self) -> Box<dyn Iterator<Item = u64> + '_>;

    fn find_solution(&mut self) -> Solution<u64> {
        assert!(self.filtered().len() != 0, "We need some caches");

        let mut improves = false;
        let mut pivot = self.filtered().len() - 1;
        loop {
            if self.try_advance_index(pivot) {
                self.advance_index(pivot);
                improves = true;
            } else if pivot > 0 {
                pivot -= 1;
            } else {
                if !improves {
                    break;
                }
                pivot = self.filtered().len() - 1;
                improves = false;
            }
        }

        Solution {
            cost: self.get_cost(),
            vertical_caches: self.vertical_caches().collect(),
            horizontal_caches: self.horizontal_caches().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Solution<N> {
    cost: u64,
    vertical_caches: Box<[N]>,
    horizontal_caches: Box<[N]>,
}

impl<N> Solution<N> {
    /// Get the solution's cost.
    pub fn cost(&self) -> u64 {
        self.cost
    }

    /// Get a reference to the solution's vertical caches.
    pub fn vertical_caches(&self) -> &[N] {
        self.vertical_caches.as_ref()
    }

    /// Get a reference to the solution's horizontal caches.
    pub fn horizontal_caches(&self) -> &[N] {
        self.horizontal_caches.as_ref()
    }
}

impl<N> PartialOrd for Solution<N>
where
    N: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl<N> Ord for Solution<N>
where
    N: Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

pub fn get_cache_locations(width: u64, height: u64, nhoriz: usize, nvert: usize) -> Solution<u64> {
    let (sol_horiz, sol_vert) = rayon::join(
        || SearcherFirstHorizontal::create(width, height, nhoriz, nvert).find_solution(),
        || SearcherFirstVertical::create(width, height, nhoriz, nvert).find_solution(),
    );

    min(sol_horiz, sol_vert)
}

#[cfg(test)]
mod tests {
    use crate::{get_cost, gridsearcher::get_cache_locations, Solution};

    #[test]
    fn test_search_100_50_3_1() {
        let sol = get_cache_locations(100, 50, 3, 1);

        assert_eq!(sol.horizontal_caches(), [34, 56, 78]);
        assert_eq!(sol.vertical_caches(), [25]);
        assert_eq!(sol.cost(), 164_053);

        check_solution(sol, 100, 50);
    }

    #[test]
    fn test_search_100_60_3_1() {
        let sol = get_cache_locations(100, 60, 3, 1);

        assert_eq!(sol.horizontal_caches(), [27, 54, 77]);
        assert_eq!(sol.vertical_caches(), [37]);
        assert_eq!(sol.cost(), 220_407);

        check_solution(sol, 100, 60);
    }

    #[test]
    fn test_search_100_80_2_2() {
        let sol = get_cache_locations(100, 80, 2, 2);

        assert_eq!(sol.horizontal_caches(), [44, 74]);
        assert_eq!(sol.vertical_caches(), [29, 58]);

        assert_eq!(sol.cost(), 350_224);

        check_solution(sol, 100, 80);
    }

    #[test]
    fn test_search_100_100_0_0() {
        let sol = get_cache_locations(100, 100, 1, 0);

        assert_eq!(sol.horizontal_caches(), [50]);
        assert_eq!(sol.vertical_caches(), []);
        assert_eq!(sol.cost(), 740_050);

        check_solution(sol, 100, 100);
    }

    fn check_solution(sol: Solution<u64>, width: u64, height: u64) {
        let full_v: Vec<_> = sol
            .vertical_caches()
            .iter()
            .chain([&height])
            .copied()
            .collect();
        let full_h: Vec<_> = sol
            .horizontal_caches()
            .iter()
            .chain([&width])
            .copied()
            .collect();

        assert_eq!(sol.cost(), get_cost(full_h.as_slice(), full_v.as_slice()));
    }
}
