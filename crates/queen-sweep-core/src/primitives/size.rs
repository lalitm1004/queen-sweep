/// Marker trait for board sizes supported by [`QueensBoard`](crate::board::QueensBoard).
///
/// A board size `N` is valid only when the array type `[(); N]` implements
/// this trait. Sizes `1` and `4..=16` are supported; any other size is
/// rejected at compile time because `[(); N]` will not implement the trait.
///
/// This bound appears throughout the crate as `where [(); N]: ValidBoardSize`.
pub trait ValidBoardSize {}

impl ValidBoardSize for [(); 1] {}

seq_macro::seq!(N in 4..=16 {
    impl ValidBoardSize for [(); N] {}
});

#[cfg(test)]
mod tests {
    use super::ValidBoardSize;

    fn assert_valid_board_size<T: ValidBoardSize>() {}

    #[test]
    fn test_valid_board_sizes() {
        assert_valid_board_size::<[(); 1]>();
        assert_valid_board_size::<[(); 4]>();
        assert_valid_board_size::<[(); 5]>();
        assert_valid_board_size::<[(); 6]>();
        assert_valid_board_size::<[(); 7]>();
        assert_valid_board_size::<[(); 8]>();
        assert_valid_board_size::<[(); 9]>();
        assert_valid_board_size::<[(); 10]>();
        assert_valid_board_size::<[(); 11]>();
        assert_valid_board_size::<[(); 12]>();
        assert_valid_board_size::<[(); 13]>();
        assert_valid_board_size::<[(); 14]>();
        assert_valid_board_size::<[(); 15]>();
        assert_valid_board_size::<[(); 16]>();
    }
}
