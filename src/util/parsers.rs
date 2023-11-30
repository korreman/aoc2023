use std::{any::TypeId, marker::PhantomData, str::FromStr};

/// An iterator through the numbers contained in a string.
/// NOTE: Currently doesn't support negative numbers with zero spaces in-between,
/// fx. "-123-345" as i32 will fail to parse.
pub struct Numbers<'a, T> {
    _phantom: PhantomData<T>,
    stream: &'a str,
}

impl<'a, T> Numbers<'a, T> {
    pub fn new(stream: &'a str) -> Self {
        Self {
            _phantom: PhantomData,
            stream,
        }
    }
}

impl<'a, T: 'static + FromStr> Iterator for Numbers<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let skip_sign = ![
            TypeId::of::<i8>(),
            TypeId::of::<i16>(),
            TypeId::of::<i32>(),
            TypeId::of::<i64>(),
            TypeId::of::<i128>(),
        ]
        .contains(&TypeId::of::<T>());
        let pattern: &[char] = if skip_sign {
            &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        } else {
            &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-']
        };
        let start = self.stream.find(pattern)?;
        self.stream = self.stream.split_at(start).1;
        let end = self
            .stream
            .find(|c| !pattern.contains(&c))
            .unwrap_or(self.stream.len());
        let (number, stream) = self.stream.split_at(end);
        self.stream = stream;

        number.parse::<T>().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test() {
        let nums = Numbers::<i32>::new("123abca321fwip-321").collect_vec();
        assert_eq!(nums, vec![123, 321, -321]);
    }
}
