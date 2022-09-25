#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, transactional};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{
		traits::{AtLeast32BitUnsigned, CheckedAdd, MaybeSerializeDeserialize, One, Zero},
		RuntimeDebug,
	};

	use codec::{Codec, Decode, Encode, MaxEncodedLen};

	const COLUMNS: u8 = 7;
	const ROWS: u8 = 6;
	const MAX_STEPS: u8 = COLUMNS * ROWS;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	pub struct Grid([GridStatus; MAX_STEPS as usize]);
	impl Default for Grid {
		fn default() -> Self {
			Grid([GridStatus::Empty; MAX_STEPS as usize])
		}
	}

	#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	pub enum GameResult {
		Init,
		// Win for creator win
		Win,
		// Lost for opponent win
		Lost,
		Draw,
	}

	impl Default for GameResult {
		fn default() -> Self {
			GameResult::Init
		}
	}

	#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	pub enum GridStatus {
		Empty,
		// creator use red, red first then black.
		Red,
		Black,
	}

	impl Default for GridStatus {
		fn default() -> Self {
			GridStatus::Empty
		}
	}

	#[derive(
		Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, MaxEncodedLen, TypeInfo,
	)]
	pub struct Game<AccountId> {
		// Creator of the game
		pub creator: AccountId,
		// Opponent also used for judge if game started.
		pub opponent: Option<AccountId>,
		// game ongoing or ended
		pub result: GameResult,
		// 1 next player is opponent, 0 next player is creator
		pub steps: u8,
		pub grid: Grid,
	}

	impl<AccountId> Game<AccountId> {
		fn new(account: AccountId) -> Self {
			Game {
				creator: account,
				opponent: None,
				result: Default::default(),
				steps: 0,
				grid: Default::default(),
			}
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type GameId: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Codec
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ One
			+ TypeInfo;

		#[pallet::constant]
		type WinReward: Get<u32>;

		#[pallet::constant]
		type DrawReward: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn games)]
	pub type Games<T: Config> = StorageMap<_, Blake2_128Concat, T::GameId, Game<T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn scores)]
	pub type Scores<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn game_id)]
	pub type GameId<T: Config> = StorageValue<_, T::GameId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New game created
		GameCreated(T::AccountId, T::GameId),
		/// Someone accept the game
		GameChallenged(T::AccountId, T::GameId),
		/// game over
		GameOver(T::GameId, GameResult),
		/// Someone played the game
		GamePlay(T::AccountId, T::GameId, u8, u8, GridStatus),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Game ID overflow
		GameIdOverflow,
		/// Game not created yet
		GameNotExists,
		/// Someone already accepted the game
		GameAlreadyInChallenged,
		/// Game challenger is invalid
		GameInvalidChallenger,
		/// Invalid next player
		NotYourTurn,
		/// Point already used
		AlreadyOccupied,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// get next game id
			let next_game_id = Self::get_next_game_id().map_err(|_| Error::<T>::GameIdOverflow)?;
			// create a new game
			let game: Game<T::AccountId> = Game::new(who.clone());

			// insert game to map
			Games::<T>::insert(next_game_id, game);
			// update current game id
			GameId::<T>::put(next_game_id);

			// emit event
			Self::deposit_event(Event::GameCreated(who, next_game_id));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn challenge(origin: OriginFor<T>, game_id: T::GameId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check game is valid
			let mut game = Games::<T>::get(&game_id).ok_or(Error::<T>::GameNotExists)?;
			// check game not challenged before
			ensure!(game.opponent == None, Error::<T>::GameAlreadyInChallenged);
			// challenge self is invalid
			ensure!(game.creator != who, Error::<T>::GameInvalidChallenger);

			// update opponent in game
			game.opponent = Some(who.clone());
			// update game
			Games::<T>::insert(game_id, game);

			// emit event
			Self::deposit_event(Event::GameChallenged(who, game_id));
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn play(
			origin: OriginFor<T>,
			game_id: T::GameId,
			column: u8,
			row: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check game
			let mut game = Games::<T>::get(&game_id).ok_or(Error::<T>::GameNotExists)?;

			// check game started via opponent
			ensure!(game.opponent != None, Error::<T>::GameIdOverflow);
			// check next stone in board

			ensure!(column < COLUMNS && row < ROWS, Error::<T>::GameIdOverflow);

			// check position is empty
			ensure!(
				game.grid.0[Self::get_grid_position(column, row)] == GridStatus::Empty,
				Error::<T>::AlreadyOccupied
			);

			// ensure right player and set the color
			let red_turn = game.steps % 2 == 0;
			if red_turn {
				ensure!(game.creator == who.clone(), Error::<T>::NotYourTurn);
				game.grid.0[Self::get_grid_position(column, row)] = GridStatus::Red;
			} else {
				ensure!(game.opponent == Some(who.clone()), Error::<T>::NotYourTurn);
				game.grid.0[Self::get_grid_position(column, row)] = GridStatus::Black;
			}

			// increment the steps
			game.steps += 1;

			// check if win
			game.result = Self::check_result(column, row, red_turn, &game.grid);

			// check if game is draw
			if game.result == GameResult::Init && game.steps == MAX_STEPS {
				game.result = GameResult::Draw;
			}

			// reward players
			Self::reward_player(game.result, game.creator.clone(), game.opponent.clone().unwrap());

			// emit event
			let color = if red_turn { GridStatus::Red } else { GridStatus::Black };
			Self::deposit_event(Event::GamePlay(who.clone(), game_id, column, row, color));

			if game.result == GameResult::Init {
				// update game
				Games::<T>::insert(game_id, game);
			} else {
				Games::<T>::remove(game_id);
				// emit event
				Self::deposit_event(Event::GameOver(game_id, game.result));
			}

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// get next game id, return zero if it is None
		fn get_next_game_id() -> Result<T::GameId, ()> {
			match Self::game_id() {
				None => Ok(Zero::zero()),
				Some(id) => id.checked_add(&One::one()).ok_or(()),
			}
		}

		/// get the index in a linear array
		fn get_grid_position(column: u8, row: u8) -> usize {
			(column * ROWS + row) as usize
		}

		/// reward players according to result
		fn reward_player(res: GameResult, creator: T::AccountId, opponent: T::AccountId) {
			match res {
				GameResult::Win => {
					let win_award = T::WinReward::get();
					Scores::<T>::mutate(creator, |score| *score = score.saturating_add(win_award));
					Scores::<T>::mutate(opponent, |score| *score = score.saturating_sub(win_award));
				},
				GameResult::Lost => {
					let win_award = T::WinReward::get();
					Scores::<T>::mutate(creator, |score| *score = score.saturating_sub(win_award));
					Scores::<T>::mutate(opponent, |score| *score = score.saturating_add(win_award));
				},
				GameResult::Draw => {
					let draw_award = T::DrawReward::get();
					Scores::<T>::mutate(creator, |score| *score = score.saturating_add(draw_award));
					Scores::<T>::mutate(opponent, |score| {
						*score = score.saturating_add(draw_award)
					});
				},
				GameResult::Init => {},
			}
		}

		/// check the game is over if latest player win
		fn check_result(column: u8, row: u8, is_red: bool, grid: &Grid) -> GameResult {
			let color = if is_red { GridStatus::Red } else { GridStatus::Black };

			// check vertical
			let mut connected = 0_u8;

			for index in (0..column).rev() {
				let pos = Self::get_grid_position(index, row);
				if grid.0[pos] == color {
					connected += 1;
				} else {
					break
				}
			}

			for index in column..COLUMNS {
				let pos = Self::get_grid_position(index, row);
				if grid.0[pos] == color {
					connected += 1;
				} else {
					break
				}
			}

			if connected >= 4 {
				if is_red {
					return GameResult::Win
				} else {
					return GameResult::Lost
				}
			}

			// check horizontal
			connected = 0_u8;

			for index in (0..row).rev() {
				let pos = Self::get_grid_position(column, index);
				if grid.0[pos] == color {
					connected += 1;
				} else {
					break
				}
			}

			for index in row..ROWS {
				let pos = Self::get_grid_position(column, index);
				if grid.0[pos] == color {
					connected += 1;
				} else {
					break
				}
			}

			if connected >= 4 {
				if is_red {
					return GameResult::Win
				} else {
					return GameResult::Lost
				}
			}

			// check diagonal
			connected = 0_u8;

			// start from new point
			let mut current_column = column;
			let mut current_row = row;

			loop {
				let pos = Self::get_grid_position(current_column, current_row);
				if color == grid.0[pos] {
					connected += 1;
				} else {
					break
				}
				if current_column == 0 || current_row == 0 {
					break
				}
				current_column -= 1;
				current_row -= 1;
			}

			// reset current column and row
			current_column = column;
			current_row = row;

			loop {
				let pos = Self::get_grid_position(current_column, current_row);
				if color == grid.0[pos] {
					connected += 1;
				} else {
					break
				}

				current_column += 1;
				current_row += 1;

				if current_column == COLUMNS || current_row == ROWS {
					break
				}
			}

			// need at least 5 connected since current one counted twice
			if connected > 4 {
				if is_red {
					return GameResult::Win
				} else {
					return GameResult::Lost
				}
			}

			// check reverse diagonal
			connected = 0_u8;

			current_column = column;
			current_row = row;

			loop {
				let pos = Self::get_grid_position(current_column, current_row);
				if color == grid.0[pos] {
					connected += 1;
				} else {
					break
				}
				if current_column == 0 {
					break
				}

				current_column -= 1;
				current_row += 1;

				if current_row == ROWS {
					break
				}
			}

			current_column = column;
			current_row = row;

			loop {
				let pos = Self::get_grid_position(current_column, current_row);
				if color == grid.0[pos] {
					connected += 1;
				} else {
					break
				}

				if current_row == 0 {
					break
				}

				current_row -= 1;
				current_column += 1;

				if current_column == COLUMNS {
					break
				}
			}

			if connected > 4 {
				if is_red {
					return GameResult::Win
				} else {
					return GameResult::Lost
				}
			}

			// game not over yet, not check draw here
			GameResult::Init
		}
	}
}
