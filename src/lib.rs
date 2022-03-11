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

pub mod cost;

pub use cost::get_cost;

#[cfg(test)]
mod tests {
    use crate::get_cost;

    #[test]
    fn square_100_100_3() {
        let w = [36u8, 73, 100];
        let h = [59, 100];

        assert_eq!(get_cost(w, h), 537857);
    }
    #[test]
    fn grid_100_80_4() {
        let h = [44u8, 74, 100];
        let w = [29, 58, 80];

        assert_eq!(get_cost(w, h), 350224);
    }
    #[test]
    fn grid_100_40_3() {
        let w = [25u8, 50, 75, 100];
        let h = [40];

        assert_eq!(get_cost(w, h), 126075);
    }
    #[test]
    fn grid_100_40_4() {
        let w = [32u8, 54, 77, 100];
        let h = [20, 40];

        assert_eq!(get_cost(w, h), 114617);
    }
}
