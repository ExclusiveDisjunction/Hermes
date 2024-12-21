/// Takes a specific number of elements out of a vec, and returns it as a concrete [type; count] type.
/// ```
/// let vec: Vec<u32> = vec![1, 2, 3, 4];
/// assert_eq!(take_from_vec!(3, vec, u32), [1, 2, 3])
/// ```
#[macro_export]
macro_rules! take_from_vec {
    ($count: expr, $vec: expr, $type: ty) => {
        {
            if $vec.len() < $count {
                panic!("expected at least {}, got {}", $count, $vec.len());
            }

            let mut result: [$type; $count] = [0; $count];
            for i in 0..$count {
                result[i] = $vec[i]
            }

            result
        }
    }
}

#[test]
pub fn test_take_from_vec() {
    let vec: Vec<u32> = vec![1, 2, 3, 4];
    assert_eq!(take_from_vec!(3, vec, u32), [1, 2, 3])
}