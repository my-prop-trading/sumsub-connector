/// SumSub verification steps (`idDocSetType`), as accepted by the reset-step endpoint. Only the
/// steps we reset are modelled; SumSub's full list also covers company, questionnaire and e-KYC
/// steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStep {
    Identity,
    ProofOfResidence,
}

impl VerificationStep {
    pub fn as_str(&self) -> &'static str {
        match self {
            VerificationStep::Identity => "IDENTITY",
            VerificationStep::ProofOfResidence => "PROOF_OF_RESIDENCE",
        }
    }
}
