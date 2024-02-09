use std::hash::Hash;

pub trait SearchIndexable<K, M: SearchMarker, N: Search<M>> {
    fn id(&self) -> K;
    fn strings(&self) -> Vec<String>;
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

pub trait SearchMarker: Sized {}

pub trait ToSearchMaker<M: SearchMarker, S: Search<M>> {
    fn to_search_marker(item: &S::Item) -> &M;
}
