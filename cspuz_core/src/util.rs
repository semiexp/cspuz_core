use std::marker::PhantomData;
use std::ops::{BitOr, BitOrAssign, Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdateStatus {
    NotUpdated,
    Updated,
    Unsatisfiable,
}

impl BitOr<UpdateStatus> for UpdateStatus {
    type Output = UpdateStatus;

    fn bitor(self, rhs: UpdateStatus) -> Self::Output {
        match (self, rhs) {
            (UpdateStatus::Unsatisfiable, _) | (_, UpdateStatus::Unsatisfiable) => {
                UpdateStatus::Unsatisfiable
            }
            (UpdateStatus::Updated, _) | (_, UpdateStatus::Updated) => UpdateStatus::Updated,
            _ => UpdateStatus::NotUpdated,
        }
    }
}

impl BitOrAssign<UpdateStatus> for UpdateStatus {
    fn bitor_assign(&mut self, rhs: UpdateStatus) {
        *self = *self | rhs;
    }
}

pub trait ConvertMapIndex {
    fn to_index(&self) -> usize;
}

pub struct ConvertMap<K: ConvertMapIndex, V: Default> {
    data: Vec<V>,
    key_type: PhantomData<K>,
    default: V,
}

impl<K: ConvertMapIndex, V: Default> ConvertMap<K, V> {
    pub fn new() -> ConvertMap<K, V> {
        ConvertMap {
            data: vec![],
            key_type: PhantomData,
            default: V::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<K: ConvertMapIndex, V: Default> Index<K> for ConvertMap<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        let index = index.to_index();
        if index < self.len() {
            &self.data[index]
        } else {
            &self.default
        }
    }
}

impl<K: ConvertMapIndex, V: Default> IndexMut<K> for ConvertMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        let index = index.to_index();
        while self.len() <= index {
            self.data.push(V::default());
        }
        &mut self.data[index]
    }
}
