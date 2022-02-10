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
#![feature(test)]
#![allow(dead_code)]

extern crate test;

use git2::*;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::rc::Rc;

mod data;

type Bytes = Rc<Vec<u8>>;

trait GetFile {
    fn get_file(&mut self, path: &str) -> Bytes;
}


struct Git {
    repo: git2::Repository,
}

impl Git {
    pub fn new(path: &str) -> Self {
        let repo = Repository::open(path).unwrap();
        Self { repo }
    }
}

impl GetFile for Git {
    fn get_file(&mut self, path: &str) -> Bytes {
        let head = self.repo.head().unwrap();
        let tree = head.peel_to_tree().unwrap();
        let entry = tree.get_path(Path::new(path)).unwrap();
        let blob = self.repo.find_blob(entry.id()).unwrap();
        Rc::new(blob.content().to_vec())
    }
}



struct FS;


impl GetFile for FS {
    fn get_file(&mut self, path: &str) -> Bytes {
        Rc::new(fs::read(path).unwrap())
    }
}


struct Cache<T: GetFile> {
    fs: T,
    cache: HashMap<String, Rc<Vec<u8>>>
}

impl<T: GetFile> Cache<T> {
    pub fn new(fs: T) -> Self {
        let cache = HashMap::default();
        Self {
            fs,
            cache
        }
    }
}

impl<T: GetFile> GetFile for Cache<T> {
    fn get_file(&mut self, path: &str) -> Bytes {
        match self.cache.get(path) {
            Some(content) => content.clone(),
            None => {
                let content = self.fs.get_file(path);
                self.cache.insert(path.to_string(), content.clone());
                content
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    use super::*;
    use data::FILES;

    const TEST_DIR: &str = "/tmp/git-bench.batsense.net";

    fn init() {
        std::env::set_current_dir(TEST_DIR).unwrap();
    }

    #[bench]
    fn bench_git(b: &mut Bencher) {
        init();
        let mut git = Git::new(TEST_DIR);

        b.iter(|| {
            FILES.iter().for_each(|file| {
                black_box(git.get_file(file));
            });
        });
    }

    #[bench]
    fn bench_fs(b: &mut Bencher) {
        init();
        let mut fs = FS{};

        b.iter(|| {
            FILES.iter().for_each(|file| {
                black_box(fs.get_file(file));
            });
        });
    }


    #[bench]
    fn bench_cache_git(b: &mut Bencher) {
        init();
        let git = Git::new(TEST_DIR);
        let mut cache = Cache::new(git);

        b.iter(|| {
            FILES.iter().for_each(|file| {
                black_box(cache.get_file(file));
            });
        });
    }

    #[bench]
    fn bench_cache_fs(b: &mut Bencher) {
        init();
        let fs = FS{};
        let mut cache = Cache::new(fs);

        b.iter(|| {
            FILES.iter().for_each(|file| {
                black_box(cache.get_file(file));
            });
        });
    }


}
