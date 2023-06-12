pub mod text_type;

pub fn unique(list: Vec<i32>) -> Vec<i32> {
    list.into_iter().fold(Vec::new(), |mut acc, item| {
        if !acc.contains(&item) {
            acc.push(item);
        }
        acc
    })
}

pub fn unique_with_hashset(list: Vec<i32>) -> Vec<i32> {
    list.into_iter()
        .fold(
            std::collections::hash_set::HashSet::new(),
            |mut acc, item| {
                acc.insert(item);
                acc
            },
        )
        .into_iter()
        .collect::<Vec<i32>>()
}

pub fn gen_unique<T: Ord>(list: Vec<T>) -> Vec<T> {
    list.into_iter().fold(Vec::new(), |mut acc, item| {
        if !acc.contains(&item) {
            acc.push(item);
        }
        acc
    })
}

pub fn built_in_unique<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort_by(|a, b| a.cmp(b));
    list.dedup();
    list
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use test_case::test_case;

    use crate::built_in_unique;
    use crate::gen_unique;
    use crate::unique;
    use crate::unique_with_hashset;

    #[test_case(vec![1,2,3,4]; "positive integers when there are no duplicates in order")]
    #[test_case(vec![1,-2,3,-4]; "positive and negative integers when there are no duplicates in order")]
    #[test_case(vec![1,5,3,4,2]; "positive integers when there are no duplicates out of order")]
    #[test_case(vec![1,-5,3,-4,-2]; "positive and negative integers when there are no duplicates out of order")]
    fn unique_int32_vec(list: Vec<i32>) {
        let actual = unique(list.clone());
        assert_eq!(list, actual)
    }

    #[test_case(vec![1,2,3,4]; "positive integers when there are no duplicates in order")]
    #[test_case(vec![1,-2,3,-4]; "positive and negative integers when there are no duplicates in order")]
    #[test_case(vec![1,5,3,4,2]; "positive integers when there are no duplicates out of order")]
    #[test_case(vec![1,-5,3,-4,-2]; "positive and negative integers when there are no duplicates out of order")]
    fn unique_i32_with_hashset(list: Vec<i32>) {
        let mut actual = unique_with_hashset(list.clone());
        actual.sort();
        let mut list = list;
        list.sort();
        assert_eq!(list, actual);
    }

    #[test_case(vec!["bird","dog","snake","mango"]; "vec of str values")]
    #[test_case(vec![1,-2,3,-4]; "positive and negative integers when there are no duplicates in order")]
    #[test_case(vec!['a','b','c','d','e']; "vec of char values")]
    #[test_case(vec![1,-5,3,-4,-2]; "positive and negative integers when there are no duplicates out of order")]
    fn gen_unique_t_vec<T: Ord + Copy + Debug>(list: Vec<T>) {
        let actual = gen_unique(list.clone());
        assert_eq!(list, actual)
    }

    #[test_case(vec!["bird","dog","snake","mango"]; "vec of str values")]
    #[test_case(vec![1,-2,3,-4]; "positive and negative integers when there are no duplicates in order")]
    #[test_case(vec!['a','b','c','d','e']; "vec of char values")]
    #[test_case(vec![1,-5,3,-4,-2]; "positive and negative integers when there are no duplicates out of order")]
    fn built_in_unique_t_vec<T: Ord + Copy + Debug>(mut list: Vec<T>) {
        list.sort();
        let actual = built_in_unique(list.clone());
        assert_eq!(list, actual)
    }

    #[test_case(vec!["bird","dog","snake","mango","snake","snake","snake"]; "duplicate words in different positions")]
    #[test_case(vec!["bird","dog","dog","dog","snake","mango","mango"]; "duplicate words in a series")]
    #[test_case(vec!["bird","bird","bird","bird","dog","snake","mango","bird","mango"]; "different words in different positions")]
    fn built_in_unique_t_vec_with_dupes(mut list: Vec<&str>) {
        list.sort();
        let actual = built_in_unique(list.clone());
        let expected = vec!["bird", "dog", "mango", "snake"];
        assert_eq!(expected, actual)
    }
}
