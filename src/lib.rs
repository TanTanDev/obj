//   Copyright 2014 Colin Sherratt
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

#![crate_name = "obj"]
#![crate_type = "lib"]

extern crate genmesh;

use std::fs::File;
use std::path::Path;
use std::io::{self, BufReader};
use std::collections::HashMap;
use std::rc::Rc;
use std::collections::vec_deque::VecDeque;

pub use obj::{Obj, Object, Group, IndexTuple};
pub use mtl::{Mtl, Material};

mod obj;
mod mtl;

pub fn load(path: &Path) -> io::Result<Obj<Rc<Material>>> {
    File::open(path).map(|f| {
        let mut f = BufReader::new(f);
        let obj = Obj::load(&mut f);

        let mut materials = HashMap::new();

        for m in obj.materials().iter() {
            let mut p = path.to_path_buf();
            p.pop();
            p.push(m);
            let file = File::open(&p).ok().expect("failed to open material");
            let mut f = BufReader::new(file);
            let m = Mtl::load(&mut f);
            for m in m.materials.into_iter() {
                materials.insert(m.name.clone(), Rc::new(m));
            }
        }

        obj.map(|g| {
            let Group {
                name,
                material,
                indices
            } = g;

            let material: Option<Rc<Material>> = match material {
                Some(m) => materials.get(&m).map(|m| m.clone()),
                None => None
            };

            Group {
                name: name,
                material: material,
                indices: indices
            }
        })
    })
}


struct Words<'a>(VecDeque<&'a str>);

fn words<'a>(s: &'a str) -> Words<'a> {
    Words(s.split(|c: char| c.is_whitespace())
           .filter(|s| !s.is_empty())
           .collect())
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> { self.0.pop_front() }
}