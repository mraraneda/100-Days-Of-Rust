fn main() {
            assert_eq!(count_progress(vec![6, 5, 4, 3, 2, 9]), 1);
            assert_eq!(count_progress(vec![3, 4, 1, 2]), 2);
            assert_eq!(count_progress(vec![10, 11, 12, 9, 10]), 3);
            assert_eq!(count_progress(vec![9, 9, 9, 9, 9]), 0);
}



fn count_progress(days: Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut win = 0;
    for (pos, progress) in days.into_iter().enumerate() {
        match pos {
            0 => {
                prev = progress;
                continue;
            }
            _ => {
                if progress > prev {
                    win += 1;
                }
                prev = progress;
            }
        }
    }
    win
}

#[cfg(test)]
mod tests {
    use crate::count_progress;

    #[test]
    fn test() {
        assert_eq!(count_progress(vec![6, 5, 4, 3, 2, 9]), 1);
        assert_eq!(count_progress(vec![3, 4, 1, 2]), 2);
        assert_eq!(count_progress(vec![10, 11, 12, 9, 10]), 3);
        assert_eq!(count_progress(vec![9, 9, 9, 9, 9]), 0);
    }
}
