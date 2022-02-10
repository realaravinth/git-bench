/*
 * git2 and file system file access benchmark
 * Copyright © 2022 Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This library is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published
 * by the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this library; if not, see <http://www.gnu.org/licenses/>.
 */

pub const FILES: [&str; 100] = [
    "10/10", "10/9", "10/8", "10/7", "10/6", "10/5", "10/4", "10/3", "10/2", "10/1", "9/10", "9/9",
    "9/8", "9/7", "9/6", "9/5", "9/4", "9/3", "9/2", "9/1", "8/10", "8/9", "8/8", "8/7", "8/6",
    "8/5", "8/4", "8/3", "8/2", "8/1", "7/10", "7/9", "7/8", "7/7", "7/6", "7/5", "7/4", "7/3",
    "7/2", "7/1", "6/10", "6/9", "6/8", "6/7", "6/6", "6/5", "6/4", "6/3", "6/2", "6/1", "5/10",
    "5/9", "5/8", "5/7", "5/6", "5/5", "5/4", "5/3", "5/2", "5/1", "4/10", "4/9", "4/8", "4/7",
    "4/6", "4/5", "4/4", "4/3", "4/2", "4/1", "3/10", "3/9", "3/8", "3/7", "3/6", "3/5", "3/4",
    "3/3", "3/2", "3/1", "2/10", "2/9", "2/8", "2/7", "2/6", "2/5", "2/4", "2/3", "2/2", "2/1",
    "1/10", "1/9", "1/8", "1/7", "1/6", "1/5", "1/4", "1/3", "1/2", "1/1",
];
