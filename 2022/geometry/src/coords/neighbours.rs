use crate::coords::Coords;

impl Coords<2> {
    pub fn neighbours(self) -> impl Iterator<Item = Coords<2>> {
        let offsets = [
            [-1, -1],
            [0, -1],
            [1, -1],
            [-1, 0],
            [1, 0],
            [-1, 1],
            [0, 1],
            [1, 1],
        ];

        offsets.into_iter().map(move |offset| self + Coords(offset))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_adjacent() {
//         assert_eq!(
//             Coords([1, 1]).adjacent().collect::<Vec<_>>(),
//             vec![
//                 Coords([0, 1]),
//                 Coords([2, 1]),
//                 Coords([1, 0]),
//                 Coords([1, 2])
//             ]
//         );
//     }
// }
