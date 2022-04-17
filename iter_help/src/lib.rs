use std::iter::Iterator;

pub enum LR<L, R> {
    L(L),
    R(R),
}

pub trait IterHelp : Iterator {
    fn partition_map<A, B, F, L, R>(self, mut predicate: F) -> (A, B)
        where Self: Sized,
              F: FnMut(Self::Item) -> LR<L, R>,
              A: Default + Extend<L>,
              B: Default + Extend<R>,
    {
        let mut left = A::default();
        let mut right = B::default();

        self.for_each(|val| match predicate(val) {
            LR::L(v) => left.extend(Some(v)),
            LR::R(v) => right.extend(Some(v)),
        });

        (left, right)
    }

    fn partition_result<A, B, T, E>(self) -> (A, B)
    where
        Self: Iterator<Item = Result<T, E>> + Sized,
        A: Default + Extend<T>,
        B: Default + Extend<E>,
    {
        self.partition_map(|r| match r {
            Ok(v) => LR::L(v),
            Err(v) => LR::R(v),
        })
    }
}

impl<T> IterHelp for T
where T: ?Sized + Iterator {

}