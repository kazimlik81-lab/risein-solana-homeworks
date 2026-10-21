use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

#[derive(Debug)]
pub enum ReviewInstruction {
    AddReview {
        title: String,
        rating: u8,
        description: String,
        location: String,
    },
    UpdateReview {
        title: String,
        rating: u8,
        description: String,
        location: String,
    },
}

#[derive(BorshDeserialize)]
struct ReviewPayload {
    title: String,
    rating: u8,
    description: String,
    location: String,
}

impl ReviewInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = ReviewPayload::try_from_slice(rest)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => Self::AddReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
                location: payload.location,
            },
            1 => Self::UpdateReview {
                title: payload.title,
                rating: payload.rating,
                description: payload.description,
                location: payload.location,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use borsh::BorshSerialize;

    #[derive(BorshSerialize)]
    struct TestPayload {
        title: String,
        rating: u8,
        description: String,
        location: String,
    }

    #[test]
    fn unpacks_location_for_add_and_update() {
        let payload = TestPayload {
            title: "Cafe North".into(),
            rating: 9,
            description: "Great coffee".into(),
            location: "Baku".into(),
        };
        let mut bytes = vec![0];
        payload.serialize(&mut bytes).unwrap();
        match ReviewInstruction::unpack(&bytes).unwrap() {
            ReviewInstruction::AddReview { location, .. } => assert_eq!(location, "Baku"),
            _ => panic!("expected add review"),
        }

        bytes[0] = 1;
        match ReviewInstruction::unpack(&bytes).unwrap() {
            ReviewInstruction::UpdateReview { location, .. } => assert_eq!(location, "Baku"),
            _ => panic!("expected update review"),
        }
    }

    #[test]
    fn rejects_payload_without_location_or_with_trailing_data() {
        let mut bytes = vec![0, 4, 0, 0, 0, b'T', b'e', b's', b't', 5, 4, 0, 0, 0, b'G', b'o', b'o', b'd'];
        assert_eq!(ReviewInstruction::unpack(&bytes).unwrap_err(), ProgramError::InvalidInstructionData);

        let payload = TestPayload {
            title: "Cafe".into(),
            rating: 8,
            description: "Nice".into(),
            location: "Baku".into(),
        };
        bytes.clear();
        bytes.push(0);
        payload.serialize(&mut bytes).unwrap();
        bytes.push(0);
        assert_eq!(ReviewInstruction::unpack(&bytes).unwrap_err(), ProgramError::InvalidInstructionData);
    }
}
