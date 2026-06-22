use std::collections::HashMap;

use crate::{Meta, Path};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct MetaMap(HashMap<Path, Vec<Meta>>);

impl MetaMap {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Path, &Vec<Meta>)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Path, &mut Vec<Meta>)> {
        self.0.iter_mut()
    }

    pub fn contains(&self, path: &Path) -> bool {
        self.0.contains_key(path)
    }

    pub fn get(&self, path: &Path) -> Option<&[Meta]> {
        self.0.get(path).map(|v| v.as_slice())
    }

    pub fn get_mut(&mut self, path: &Path) -> Option<&mut Vec<Meta>> {
        self.0.get_mut(path)
    }

    pub fn insert(&mut self, path: Path, items: Vec<Meta>) -> Option<Vec<Meta>> {
        self.0.insert(path, items)
    }

    pub fn append(&mut self, path: Path, meta: &mut Vec<Meta>) {
        let items = self.0.entry(path).or_insert(vec![]);
        items.append(meta);
    }

    pub fn push(&mut self, path: Path, meta: Meta) {
        let items = self.0.entry(path).or_insert(vec![]);
        items.push(meta);
    }

    pub fn delete(&mut self, path: &Path) -> Option<Vec<Meta>> {
        self.0.remove(path)
    }
}

impl From<HashMap<Path, Vec<Meta>>> for MetaMap {
    fn from(value: HashMap<Path, Vec<Meta>>) -> Self {
        Self(value)
    }
}

impl Extend<Meta> for MetaMap {
    fn extend<T: IntoIterator<Item = Meta>>(&mut self, iter: T) {
        for meta in iter {
            if let Meta::List(list) = meta {
                self.extend(list.items.into_inner().into_iter());
            } else {
                self.push(meta.path().clone(), meta);
            }
        }
    }
}
