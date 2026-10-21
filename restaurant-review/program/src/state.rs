use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{IsInitialized, Sealed};
use thiserror::Error;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AccountState {
    pub is_initialized: bool,
    pub rating: u8,
    pub description: String,
    pub title: String,
    pub location: String,
}

impl Sealed for AccountState {}

impl IsInitialized for AccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Account not initialized yet")]
    UninitializedAccount,

    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,

    #[error("Rating greater than 10 or less than 1")]
    InvalidRating,

    #[error("Review text exceeds the allocated account size")]
    ReviewTooLong,
}

impl From<ReviewError> for ProgramError {
    fn from(e: ReviewError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_state_round_trips_location() {
        let state = AccountState {
            is_initialized: true,
            rating: 9,
            description: "Great coffee".into(),
            title: "Cafe North".into(),
            location: "Baku".into(),
        };
        let encoded = borsh::to_vec(&state).unwrap();
        let decoded = AccountState::try_from_slice(&encoded).unwrap();
        assert!(decoded.is_initialized);
        assert_eq!(decoded.rating, 9);
        assert_eq!(decoded.description, "Great coffee");
        assert_eq!(decoded.title, "Cafe North");
        assert_eq!(decoded.location, "Baku");
    }
}
