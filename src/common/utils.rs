#[cfg(test)]
macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

#[cfg(test)]
pub(crate) use vec_of_strings;
