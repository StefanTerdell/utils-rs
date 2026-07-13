pub trait CollectByKey<Item, Key, Output> {
    fn collect_by_key(self, f: impl Fn(&Item) -> Key) -> Output;
}

macro_rules! impl_for {
    ($map_type:ty, $list_type:ty, $add_to_list_method:ident) => {
        impl_for!($map_type, $list_type, $add_to_list_method, From::<I>);
    };

    ($map_type:ty, $list_type:ty, $add_to_list_method:ident, $item_type_bounds:expr) => {
        paste::paste! {
            impl<T: IntoIterator<Item = I>, I: $item_type_bounds, K: Eq + std::hash::Hash> CollectByKey<I, K, $map_type<K, $list_type<I>>> for T {
                fn collect_by_key(self, f: impl Fn(&I) -> K) -> $map_type<K, $list_type<I>> {
                    self.into_iter()
                        .fold($map_type::default(), |mut acc, curr| {
                            let key = f(&curr);
                            match acc.get_mut(&key) {
                                Some(list) => {
                                    list.$add_to_list_method(curr);
                                }
                                #[allow(clippy::vec_init_then_push)]
                                None => {
                                    let mut list = $list_type::default();
                                    list.$add_to_list_method(curr);
                                    acc.insert(key, list);
                                }
                            }
                            acc
                        })
                }
            }
        }
    };
}

impl_for!(std::collections::HashMap, Vec, push);
impl_for!(
    std::collections::HashMap,
    std::collections::HashSet,
    insert,
    Eq + std::hash::Hash
);
#[cfg(feature = "indexmap")]
impl_for!(
    std::collections::HashMap,
    indexmap::IndexSet,
    insert,
    Eq + std::hash::Hash
);

#[cfg(feature = "indexmap")]
impl_for!(indexmap::IndexMap, Vec, push);
#[cfg(feature = "indexmap")]
impl_for!(
    indexmap::IndexMap,
    std::collections::HashSet,
    insert,
    Eq + std::hash::Hash
);
#[cfg(feature = "indexmap")]
impl_for!(
    indexmap::IndexMap,
    indexmap::IndexSet,
    insert,
    Eq + std::hash::Hash
);
