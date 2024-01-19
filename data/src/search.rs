use skill::Target;
use std::hash::Hash;

pub trait SearchIndexable<K, M: SearchMarker, N: Search<M>> {
    fn id(&self) -> K;
    fn strings(&self) -> Vec<String>;
    fn lift(&self) -> M;
}

pub trait Search<M: SearchMarker>: Sized {
    type Key: Ord + Clone + Hash;
    type Item: SearchIndexable<Self::Key, M, Self>;
    type Repository: Repository<Self::Key, Self::Item>;
    type Marker: SearchMarker;
}

pub trait Repository<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a K> + 'a>
    where
        K: 'a;
}

pub trait SearchMarker: Sized + Search<Self> {
    fn new(item: &Self::Item) -> Self::Marker;
}
