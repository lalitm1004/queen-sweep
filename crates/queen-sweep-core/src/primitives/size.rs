pub trait ValidBoardSize {}

impl ValidBoardSize for [(); 1] {}

seq_macro::seq!(N in 4..=16 {
    impl ValidBoardSize for [(); N] {}
});
