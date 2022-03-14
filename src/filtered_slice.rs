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

use std::ops::{Index, IndexMut};

use itertools::Itertools;

#[derive(Debug)]
pub struct FilteredSlice<E, Idx> {
    elements: E,
    index_translations: Box<[Idx]>,
}

impl<E, Idx> Index<usize> for FilteredSlice<E, Idx>
where
    E: Index<Idx>,
    Idx: Copy,
{
    type Output = E::Output;

    fn index(&self, index: usize) -> &Self::Output {
        self.elements.index(self.index_translations[index])
    }
}

impl<E, Idx> IndexMut<usize> for FilteredSlice<E, Idx>
where
    E: IndexMut<Idx>,
    Idx: Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let real_index = self.index_translations[index];
        self.elements.index_mut(real_index)
    }
}

impl<E, Idx> FilteredSlice<E, Idx> {
    pub fn len(&self) -> usize {
        self.index_translations.len()
    }

    pub fn create<I>(elements: E, valid_indices: I) -> Self
    where
        E: Index<Idx>,
        I: IntoIterator<Item = Idx>,
        Idx: Ord,
    {
        let index_translations: Box<_> = valid_indices.into_iter().sorted().collect();

        FilteredSlice {
            elements,
            index_translations,
        }
    }

    pub fn get_raw_ref(&self) -> &E {
        &self.elements
    }
}

#[cfg(test)]
mod tests {
    use super::FilteredSlice;

    #[test]
    fn create_view_from_vector() {
        let array = vec![0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut res = FilteredSlice::create(array, [1, 4, 7]);

        assert_eq!(res.len(), 3);
        assert_eq!(res[1], 4);
        res[2] = 5000;
        assert_eq!(res.get_raw_ref()[7], 5000);
    }

    #[test]
    fn create_view_from_slice() {
        let array = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut res = FilteredSlice::create((&array[2..6]).to_owned(), [0, 1, 3]);

        assert_eq!(res.len(), 3);
        assert_eq!(res[1], 3);
        res[2] = 1000;
        assert_eq!(res[2], 1000);
        assert_eq!(res.get_raw_ref()[3], 1000);
    }
}
