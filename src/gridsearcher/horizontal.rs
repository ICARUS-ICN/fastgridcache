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

use crate::filtered_slice::FilteredSlice;

use super::{Problem, Searcher};

#[derive(Debug)]
pub struct SearcherFirstHorizontal {
    problem: Problem,
}

impl SearcherFirstHorizontal {
    pub fn create(width: u64, height: u64, nhoriz: usize, nvert: usize) -> Self
where {
        assert!(nhoriz >= nvert);
        assert!(width as usize > nhoriz);
        assert!(height as usize > nvert);

        let caches: Vec<_> = [0]
            .into_iter() // Origin
            .chain(
                // While there are vertical caches
                (0..nvert).flat_map(|x| [2 * x + 1, 2 * (x + 1)].into_iter()),
            )
            .map(|x| x as u64)
            .chain(
                // This is for horizontal ones
                (0..nhoriz - nvert).flat_map(|x| [(2 * nvert + x + 1) as u64, height].into_iter()),
            )
            .chain([width, height])
            .collect();

        let valid: Vec<_> = (0..nvert)
            .map(|x| 2 * (x + 1))
            .chain((0..nhoriz).map(|x| 2 * x + 1))
            .collect();

        let filtered = FilteredSlice::create(caches, valid);
        Self {
            problem: Problem {
                nhoriz,
                nvert,
                width,
                filtered,
            },
        }
    }
}

impl Searcher for SearcherFirstHorizontal {
    fn vertical_caches(&self) -> Box<dyn Iterator<Item = u64> + '_> {
        Box::new(
            (0..self.problem.nvert)
                .map(|x| 2 * (x + 1))
                .map(|i| self.problem.filtered.get_raw_ref()[i]),
        )
    }

    fn horizontal_caches(&self) -> Box<dyn Iterator<Item = u64> + '_> {
        Box::new(
            (0..self.problem.nhoriz)
                .map(|x| 2 * x + 1)
                .map(|i| self.problem.filtered.get_raw_ref()[i]),
        )
    }

    fn width(&self) -> u64 {
        self.problem.width
    }

    fn filtered_mut(&mut self) -> &mut FilteredSlice<Vec<u64>, usize> {
        self.problem.filtered_mut()
    }

    fn filtered(&self) -> &FilteredSlice<Vec<u64>, usize> {
        self.problem.filtered()
    }
}

#[cfg(test)]
mod tests {
    use crate::gridsearcher::{horizontal::SearcherFirstHorizontal, Searcher};

    #[test]
    fn test_search_100_60_3_1() {
        let mut searcher = SearcherFirstHorizontal::create(100, 60, 3, 1);
        let sol = searcher.find_solution();

        assert_eq!(sol.horizontal_caches(), [27, 54, 77]);
        assert_eq!(sol.vertical_caches(), [37]);
        assert_eq!(sol.cost(), 220_407);
    }

    #[test]
    fn test_search_100_100_0_0() {
        let mut searcher = SearcherFirstHorizontal::create(100, 100, 1, 0);
        let sol = searcher.find_solution();

        assert_eq!(sol.horizontal_caches(), [50]);
        assert_eq!(sol.vertical_caches(), []);
        assert_eq!(sol.cost(), 740_050);
    }
}
