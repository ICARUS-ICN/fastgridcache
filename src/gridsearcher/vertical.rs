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
pub struct SearcherFirstVertical {
    problem: Problem,
}
impl SearcherFirstVertical {
    pub fn create(width: u64, height: u64, nhoriz: usize, nvert: usize) -> Self {
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
                (0..nhoriz - nvert).flat_map(|x| [height, (2 * nvert + x + 1) as u64].into_iter()),
            )
            .chain([height, width])
            .collect();

        let valid: Vec<_> = (0..nvert)
            .map(|x| 2 * x + 1)
            .chain((0..nhoriz).map(|x| 2 * (x + 1)))
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

impl Searcher for SearcherFirstVertical {
    fn vertical_caches(&self) -> Box<dyn Iterator<Item = u64> + '_> {
        Box::new(
            (0..self.problem.nvert)
                .map(|x| 2 * x + 1)
                .map(|i| self.problem.filtered.get_raw_ref()[i]),
        )
    }

    fn horizontal_caches(&self) -> Box<dyn Iterator<Item = u64> + '_> {
        Box::new(
            (0..self.problem.nhoriz)
                .map(|x| 2 * (x + 1))
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
    use crate::gridsearcher::{vertical::SearcherFirstVertical, Searcher};

    #[test]
    fn test_1_0() {
        let searcher = SearcherFirstVertical::create(100, 50, 1, 0);
        assert_eq!(
            searcher.problem.filtered.get_raw_ref(),
            &vec![0, 50, 1, 50, 100]
        );
    }

    #[test]
    fn test_5_0() {
        let searcher = SearcherFirstVertical::create(100, 50, 5, 0);
        assert_eq!(
            searcher.problem.filtered.get_raw_ref(),
            &vec![0, 50, 1, 50, 2, 50, 3, 50, 4, 50, 5, 50, 100]
        );
        for e in 0..5 {
            assert_eq!(searcher.problem.filtered[e] as usize, e + 1);
        }
    }

    #[test]
    fn test_5_2() {
        let searcher = SearcherFirstVertical::create(100, 50, 5, 2);
        assert_eq!(
            searcher.problem.filtered.get_raw_ref(),
            &vec![0, 1, 2, 3, 4, 50, 5, 50, 6, 50, 7, 50, 100]
        );
        for e in 0..7 {
            assert_eq!(searcher.problem.filtered[e] as usize, e + 1);
        }
    }

    #[test]
    fn test_search_100_50_3_1() {
        let mut searcher = SearcherFirstVertical::create(100, 50, 3, 1);
        let sol = searcher.find_solution();

        assert_eq!(sol.horizontal_caches(), [34, 56, 78]);
        assert_eq!(sol.vertical_caches(), [25]);
        assert_eq!(sol.cost(), 164_053);
    }

    #[test]
    fn test_search_100_80_2_2() {
        let mut searcher = SearcherFirstVertical::create(100, 80, 2, 2);
        let sol = searcher.find_solution();

        assert_eq!(sol.horizontal_caches(), [44, 74]);
        assert_eq!(sol.vertical_caches(), [29, 58]);

        assert_eq!(sol.cost(), 350_224);
    }

    #[test]
    fn test_search_100_100_0_0() {
        let mut searcher = SearcherFirstVertical::create(100, 100, 1, 0);
        let sol = searcher.find_solution();

        assert_eq!(sol.horizontal_caches(), [50]);
        assert_eq!(sol.vertical_caches(), []);
        assert_eq!(sol.cost(), 740_050);
    }
}
