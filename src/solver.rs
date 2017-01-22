use std::fmt;

pub trait GameState {
    type Action;

    fn is_win(&self) -> bool;
    fn actions(&self) -> Vec<Self::Action>;

    fn apply(&mut self, &Self::Action);
    fn revert(&mut self, &Self::Action);
}

pub fn solve<G: GameState + fmt::Display>(game_state: &mut G) -> bool
where G::Action: fmt::Debug {
    if game_state.is_win() {
        return true;
    }


    // TODO: enable multiple debug levels based on flags.

    // println!("{}\n", game_state);
    for a in game_state.actions() {
        // println!("Trying {:?} ({} / {})", a, i+1, possible_actions.len());
        game_state.apply(&a);
        if solve(game_state) {
            return true;
        }
        // println!("Blocked, reverting {:?}", a);
        game_state.revert(&a);
    }
    return false;
}
