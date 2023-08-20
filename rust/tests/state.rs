#[cfg(test)]
mod tests {
    use gol::app::{InitPattern, State};
    use std::collections::HashSet;

    #[test]
    fn it_steps_through_the_simple_extinct_state() {
        let pattern = InitPattern::Pattern("simple-extinct".to_string());
        let mut state = State::new(pattern);
        assert_eq!(state.alive, 3);
        assert_eq!(state.points(), &HashSet::from([(0, 1), (-1, 0), (1, 0)]));
        state.step();
        assert_eq!(state.alive, 2);
        assert_eq!(state.points(), &HashSet::from([(0, 1), (0, 0)]));
        state.step();
        assert_eq!(state.alive, 0);
        assert_eq!(state.points(), &HashSet::new());
    }

    #[test]
    fn it_steps_through_the_simple_stable_state() {
        let stable = &HashSet::from([(0, 0), (0, 1), (-1, 0), (-1, 1)]);
        let pattern = InitPattern::Pattern("simple-stable".to_string());
        let mut state = State::new(pattern);
        assert_eq!(state.alive, 4);
        assert_eq!(state.points(), stable);
        state.step();
        assert_eq!(state.alive, 4);
        assert_eq!(state.points(), stable);
    }
}
