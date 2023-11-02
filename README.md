Fast In-Axis Cache Placement Algorithm for Grid-Like NDN Networks 
===

[![build](https://github.com/ICARUS-ICN/fastgridcache/actions/workflows/build.yml/badge.svg)](https://github.com/ICARUS-ICN/fastgridcache/actions/workflows/build.yml)

Overview
---

This is a tool to compute the optimal cache locations in a grid-like topology assuming 
that the caches are to be placed in the axis and that there is a producer at the center
of the grid.

The behavior of the in-axis placement strategy is described in the article
[Cache Placement in an NDN Based LEO Satellite Network
Constellation](https://doi.org/10.1109/TAES.2022.3227530).

Usage
---

    fastgrid OPTIONS

### Options:


    -c, --hide-cost
    -e, --height <HEIGHT>      Height of the network grid
        --help                 Print help information
    -n, --ncaches <NCACHES>    Number of caches [default: 0]
    -s, --show-caches
    -V, --version              Print version information
    -w, --width <WIDTH>        Width of the network grid


---
### Legal:
Copyright ⓒ 2021–2023 Universidade de Vigo<br>
Author: Miguel Rodríguez Pérez <miguel@det.uvigo.gal>.<br>
This software is licensed under the GNU General Public License, version 3 (GPL-3.0) or later. For information see LICENSE.


Project PID2020-113240RB-I00 financed by MCIN/ AEI/10.13039/501100011033.
![MCIN-AEI Logo](https://icarus.det.uvigo.es/assets/img/logo-mcin-aei.png)
