pub trait SearchIndexable<K, M: SearchMarker> {
    fn id(&self) -> K;
    fn strings(&self) -> Vec<String>;
    fn lift(&self) -> M {
        todo!()
    }
}

pub trait Search<M: SearchMarker> {
    type Key: Ord + Clone;
    type Item: SearchIndexable<Self::Key, M>;
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
    fn new(&self, item: Self::Item) -> Self;
}
