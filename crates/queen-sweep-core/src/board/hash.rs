/// Computes a 64-bit hash of the board state from per-row queen positions.
///
/// # Strategy
///
/// - For `N <= 15`, a collision-free positional encoding in base 16 is used.
///   This is a bijection over all possible row configurations and fits in `u64`.
/// - For `N >= 16`, a 64-bit FNV-1a hash is used instead.
///   A collision-free encoding is mathematically impossible because
///   `(N + 1)^N > 2^64`. The FNV variant provides fast, well-distributed
///   probabilistic hashing.
///
/// # Determinism
///
/// The hash is fully deterministic and `const`-evaluatable.
///
/// # Collision Guarantees
///
/// - `N <= 15` → collision-free.
/// - `N >= 16` → 64-bit non-cryptographic hash; collisions are possible.
pub const fn compute_board_hash<const N: usize>(queen_col_per_row: &[Option<usize>; N]) -> u64 {
    if N <= 15 {
        compute_board_hash_small::<N>(queen_col_per_row)
    } else {
        compute_board_hash_large::<N>(queen_col_per_row)
    }
}

/// Computes a fast 64-bit FNV-1a hash for large boards (`N >= 16`).
///
/// # Algorithm
///
/// This uses the 64-bit Fowler–Noll–Vo (FNV-1a) hash:
///
/// ```text
/// hash = (hash ^ byte) * FNV_PRIME
/// ```
///
/// Constants:
///
/// - Offset basis: `0xcbf29ce484222325`
/// - Prime:        `0x00000100000001b3`
///
/// Each row contributes one byte:
///
/// - `0..=N-1` → column index
/// - `0xFF`    → no queen in row
const fn compute_board_hash_large<const N: usize>(queen_col_per_row: &[Option<usize>; N]) -> u64 {
    let mut hash = 0xcbf29ce484222325;
    let prime = 0x00000100000001b3;

    let mut row = 0;
    while row < N {
        let digit = match queen_col_per_row[row] {
            Some(col) => col as u64,
            None => 0xFF,
        };

        hash ^= digit;
        hash = hash.wrapping_mul(prime);

        row += 1;
    }

    hash
}

/// Computes a collision-free encoding of the board state from the per-row
/// queen positions.
///
/// # Encoding
///
/// Each row contributes one digit in base 16:
///
/// - `0..=N-1` → queen present at that column index
/// - `N`       → no queen in that row
///
/// The final hash is the interpretation of this digit sequence as a number
/// in base 16, with row `0` as the most significant digit.
///
/// Formally:
///
/// ```text
/// hash = Σ digit[row] * 16^(N - 1 - row)
/// ```
///
/// This encoding is:
///
/// - Collision-free for `N <= 15`
/// - Deterministic
/// - `const`-evaluatable
/// - O(N) time
const fn compute_board_hash_small<const N: usize>(queen_col_per_row: &[Option<usize>; N]) -> u64 {
    let mut hash = 0u64;
    let mut row = 0;

    while row < N {
        let digit = match queen_col_per_row[row] {
            Some(col) => col as u64,
            None => N as u64,
        };

        hash += digit * RADIX_WEIGHTS[N - row - 1];
        row += 1;
    }

    hash
}

/// Precomputed powers of 16, from `16^0` through `16^14`.
///
/// These weights support collision-free positional encoding for boards
/// with `N <= 15`.
///
/// The weights are evaluated at compile time and reused by every hash
/// computation.
const RADIX_WEIGHTS: [u64; 15] = {
    let mut weights = [0u64; 15];
    let mut i = 0;

    while i < 15 {
        weights[i] = if i == 0 { 1 } else { weights[i - 1] * 16 };

        i += 1;
    }

    weights
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_small_board_known_hashes() {
        assert_eq!(compute_board_hash::<0>(&[]), 0);

        assert_eq!(compute_board_hash::<1>(&[Some(0)]), 0);
        assert_eq!(compute_board_hash::<1>(&[None]), 1);

        assert_eq!(compute_board_hash::<2>(&[Some(0), None]), 2);
        assert_eq!(compute_board_hash::<2>(&[None, Some(0)]), 32);
        assert_eq!(compute_board_hash::<2>(&[Some(0), Some(1)]), 1);
        assert_eq!(compute_board_hash::<2>(&[Some(1), Some(0)]), 16);
        assert_eq!(compute_board_hash::<2>(&[None, None]), 34);
    }

    #[test]
    fn test_small_board_hash_is_deterministic() {
        let board = [Some(2), None, Some(0), Some(1)];

        let first = compute_board_hash(&board);
        let second = compute_board_hash(&board);

        assert_eq!(first, second);
    }

    #[test]
    fn test_small_board_distinguishes_empty_rows() {
        let a = [Some(0), None, Some(1)];
        let b = [None, Some(0), Some(1)];
        let c = [Some(0), Some(1), None];

        assert_ne!(compute_board_hash(&a), compute_board_hash(&b));
        assert_ne!(compute_board_hash(&a), compute_board_hash(&c));
        assert_ne!(compute_board_hash(&b), compute_board_hash(&c));
    }

    fn enumerate_valid_states<const N: usize>(
        row: usize,
        board: &mut [Option<usize>; N],
        used_columns: &mut [bool; N],
        hashes: &mut HashSet<u64>,
        count: &mut usize,
    ) {
        if row == N {
            let hash = compute_board_hash(board);

            assert!(
                hashes.insert(hash),
                "Hash collision for board state: {:?}",
                board
            );

            *count += 1;
            return;
        }

        // Enumerate the state where this row has no queen.
        board[row] = None;

        enumerate_valid_states(row + 1, board, used_columns, hashes, count);

        // Enumerate every unused column for this row.
        let mut col = 0;

        while col < N {
            if !used_columns[col] {
                board[row] = Some(col as usize);
                used_columns[col] = true;

                enumerate_valid_states(row + 1, board, used_columns, hashes, count);

                used_columns[col] = false;
            }

            col += 1;
        }

        board[row] = None;
    }

    fn assert_collision_free_for_board_size<const N: usize>() {
        let mut board = [None; N];
        let mut used_columns = [false; N];
        let mut hashes = HashSet::new();
        let mut count = 0;

        enumerate_valid_states(0, &mut board, &mut used_columns, &mut hashes, &mut count);

        assert_eq!(hashes.len(), count);
    }

    #[test]
    fn test_small_board_collision_freedom() {
        seq_macro::seq!(N in 0..=5 {
            assert_collision_free_for_board_size::<N>();
        });
    }

    #[test]
    fn test_large_board_fnv1a_known_hashes() {
        let empty_columns = [Some(0); 16];
        let empty_rows = [None; 16];

        assert_eq!(compute_board_hash(&empty_columns), 0x88201fb960ff6465);

        assert_eq!(compute_board_hash(&empty_rows), 0xd6607508f5a1e855);
    }

    #[test]
    fn test_large_board_hash_is_deterministic() {
        let board = [
            Some(0),
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            Some(14),
            Some(15),
        ];

        assert_eq!(compute_board_hash(&board), compute_board_hash(&board));
    }
}
