use super::StrategySolver;
use helper::Unsolvable;

/// The strategies that can be used with the `StrategySolver` to find hints, solve or grade a `Sudoku`.
/// May be expanded in the future.
#[derive(Debug, Clone)]
pub enum Strategy {
    NakedSingles,
    HiddenSingles,
    LockedCandidates,
	NakedPairs,
	NakedTriples,
	NakedQuads,
	HiddenPairs,
	HiddenTriples,
	HiddenQuads,
    XWing,
    Swordfish,
    Jellyfish,
    SinglesChain,
    #[doc(hidden)] __NonExhaustive
}

impl Strategy {
	pub const ALL: &'static [Strategy] = &[
		                            // difficulty as assigned by
									// SudokuExplainer
		Strategy::NakedSingles,     // 23
		Strategy::HiddenSingles,    // 15
		Strategy::LockedCandidates, // 28
		Strategy::NakedPairs,       // 30
		Strategy::XWing,            // 32
		Strategy::HiddenPairs,      // 34
		Strategy::NakedTriples,     // 36
		Strategy::Swordfish,        // 38
		Strategy::HiddenTriples,    // 40
		Strategy::NakedQuads,       // 50
		Strategy::Jellyfish,        // 52
		Strategy::HiddenQuads,      // 54
		//Strategy::SinglesChain,
	];

	// is_first_strategy is an optimization hint
	// it doesn't need to be used
	pub(crate) fn deduce(&self, state: &mut StrategySolver, stop_after_first: bool, is_first_strategy: bool) -> Result<(), Unsolvable> {
		use self::Strategy::*;
        match *self {
            NakedSingles if !stop_after_first && is_first_strategy => state._update_cell_poss_house_solved(true),
			NakedSingles => state.find_naked_singles(stop_after_first),
			HiddenSingles => state.find_hidden_singles(stop_after_first),
			LockedCandidates => state.find_locked_candidates(stop_after_first),
			NakedPairs => state.find_naked_subsets(2, stop_after_first),
			NakedTriples => state.find_naked_subsets(3, stop_after_first),
			NakedQuads => state.find_naked_subsets(4, stop_after_first),
			HiddenPairs => state.find_hidden_subsets(2, stop_after_first),
			HiddenTriples => state.find_hidden_subsets(3, stop_after_first),
			HiddenQuads => state.find_hidden_subsets(4, stop_after_first),
			XWing => { state.find_xwings(stop_after_first); Ok(()) },
			Swordfish => { state.find_swordfish(stop_after_first); Ok(()) },
			Jellyfish => { state.find_jellyfish(stop_after_first); Ok(()) },
			SinglesChain if !stop_after_first => state.find_singles_chain(), // TODO: Implement non-eager SinglesChain
            _ => unimplemented!(),
        }
	}

    pub(crate) fn deduce_one(&self, state: &mut StrategySolver) -> Result<(), Unsolvable> {
        self.deduce(state, true, false)
    }

    pub(crate) fn deduce_all(&self, state: &mut StrategySolver, is_first_strategy: bool) -> Result<(), Unsolvable> {
        self.deduce(state, false, is_first_strategy)
    }
}
