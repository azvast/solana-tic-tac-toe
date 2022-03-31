use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_tic_tac_toe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct Game {
    players: [Pubkey; 2],           // 32 * 2 = 64
    turn: u8,                       // 1
    board: [[Option<Sign>; 3]; 3],  // 3 * 3 * (1 + 1) = 18
    state: GameState,               // 32 + 1
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey},
}

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    FromPrimitive,
    ToPrimitive,
    PartialEq,
    Eq,
    Copy,
    Clone
)]
pub enum Sign {
    X,
    O
}

impl Game {
    pub const MAXIMUM_SIZE: usize = (32 * 2) + 1 + (3 * 3 * (1 + 1)) + (32 + 1);

    pub fn start(&mut self, players: [Pubkey; 2]) -> Result<()> {
        require_eq(self.turn, 0, TicTacToeError::GameAlreadyStarted);
        self.players = players;
        self.turn = 1;
        Ok(());
    }

    pub fn is_active(&self) -> bool {
        self.state == GameState::Active
    }
}