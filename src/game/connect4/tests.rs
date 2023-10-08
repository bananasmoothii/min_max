#[cfg(test)]
mod p4_tests {
    use std::num::{NonZeroU8, NonZeroUsize};

    use strum::IntoEnumIterator;

    use crate::game::connect4::count_direction::CountDirection;
    use crate::game::connect4::iteration::P4IteratorType;
    use crate::game::connect4::Power4;
    use crate::game::Game;
    use crate::min_max::node::GameNode;

    #[test]
    fn lines_passing_at() {
        let power4 = Power4::new();
        let lines = power4.lines_passing_at_longer_4((0, 0));
        assert_eq!(lines.len(), 3);
        let lines = power4.lines_passing_at_longer_4((2, 3));
        assert_eq!(
            lines
                .iter()
                .map(|iter| (iter.x, iter.y))
                .collect::<Vec<_>>(),
            vec![(0, 2), (3, 0), (1, 0), (0, 5)]
        );
        let lines = power4
            .lines_passing_at_longer_4((3, 4))
            .iter()
            .map(|iter| iter.iterator_type)
            .collect::<Vec<_>>();
        for iterator_type in P4IteratorType::iter() {
            assert!(
                lines.contains(&iterator_type),
                "Iterator type {:?} not found",
                iterator_type
            );
        }
    }

    #[test]
    fn get_winner() {
        let mut power4 = Power4::new();
        let p1 = NonZeroU8::new(1).unwrap();
        let p2 = NonZeroU8::new(2).unwrap();

        for _ in 1..=2 {
            for column in 1..=7 {
                power4.play_usize(p1, column).unwrap();
            }
        }
        power4.play_usize(p1, 2).unwrap();
        power4.play_usize(p1, 3).unwrap();
        power4.play_usize(p1, 4).unwrap();
        power4.play_usize(p1, 3).unwrap();
        power4.play_usize(p1, 4).unwrap();
        power4.play_usize(p1, 4).unwrap();

        power4.play_usize(p2, 1).unwrap();
        power4.play_usize(p2, 2).unwrap();
        power4.play_usize(p2, 3).unwrap();
        power4.play_usize(p2, 4).unwrap();

        power4.print();
        println!();

        assert_eq!(power4.get_winner(), Some(p2));
    }

    #[test]
    fn min_max_should_not_help_winning() {
        let mut power4 = Power4::new();
        let p1 = NonZeroU8::new(1).unwrap();
        let p2 = NonZeroU8::new(2).unwrap();

        power4.play_usize(p2, 3).unwrap();
        power4.play_usize(p2, 4).unwrap();

        for _ in 0..3 {
            power4.play_usize(p2, 5).unwrap();
        }

        power4.play_usize(p1, 2).unwrap();
        power4.play_usize(p1, 3).unwrap();
        power4.play_usize(p1, 5).unwrap();
        power4.play_usize(p1, 6).unwrap();

        power4.print();
        // here we do not want to play 4, as it would make us loose

        println!();
        let mut game_tree = GameNode::new_root(power4.clone(), p2, 0);
        game_tree.explore_children(p2, 2, 0);
        let wrong_play = NonZeroUsize::new(4).unwrap();
        let wrong_chosen_node = &game_tree
            .children()
            .iter()
            .find(|(play, _)| play == &wrong_play)
            .unwrap()
            .1;
        assert_ne!(wrong_chosen_node.weight().unwrap(), 0);
    }

    #[test]
    fn get_winner_2() {
        let mut power4 = Power4::new();
        let p1 = NonZeroU8::new(1).unwrap();
        let p2 = NonZeroU8::new(2).unwrap();

        // AI played at 0 here:
        // - - - - - 2 -
        // - - - - - 2 -
        // - - 1 - - 2 -
        // - 1 2 1 1 1 2
        // 2 1 1 2 2 1 1
        // 2 2 2 1 1 1 2

        let plays = [
            (p2, 1),
            (p2, 2),
            (p2, 3),
            (p1, 4),
            (p2, 1),
            (p1, 2),
            (p1, 3),
            (p2, 4),
            (p1, 2),
            (p2, 3),
            (p1, 3),
            (p1, 4),
            (p2, 1),
            (p1, 1),
        ];

        for (player, column) in plays.iter() {
            power4.play_usize(*player, *column).unwrap();
        }
        power4.print();

        assert_eq!(power4.last_played_coords.unwrap(), (2, 0));
        assert_eq!(power4.lines_passing_at_longer_4((2, 0)).len(), 3);

        assert_eq!(power4.get_winner(), Some(p1));

        power4.print();
        println!();
        // let mut game_tree = GameNode::new_root(connect4.clone(), p2, 0);
        // game_tree.explore_children(p2, 2, 0);
        // println!("Tree:\n {}", game_tree.debug(3));
        // let wrong_chosen_node = game_tree.children().get(&0usize).unwrap();
        // assert_ne!(wrong_chosen_node.weight().unwrap(), 0);
    }

    #[test]
    fn count_in_direction() {
        let mut power4 = Power4::new();
        let p1 = NonZeroU8::new(1).unwrap();
        let p2 = NonZeroU8::new(2).unwrap();

        power4.play_usize(p1, 1).unwrap();
        power4.play_usize(p1, 2).unwrap();
        power4.play_usize(p1, 3).unwrap();
        power4.play_usize(p1, 4).unwrap();

        power4.print();

        assert_eq!(
            power4.count_in_direction((5, 2), CountDirection::HorizontalLeft, 10),
            2
        );
        assert_eq!(
            power4.count_in_direction((5, 2), CountDirection::HorizontalLeft, 1),
            1
        );
    }
}
