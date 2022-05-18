use core::{ops::Mul, fmt::Debug};

use thiserror::Error;

use ff::{Field, PrimeField};
use group::{Group, GroupOps, ScalarMul};

pub use multiexp::multiexp_vartime;

pub mod key_gen;
pub mod algorithm;
pub mod sign;
use sign::lagrange;

/// Set of errors for curve-related operations, namely encoding and decoding
#[derive(Error, Debug)]
pub enum CurveError {
  #[error("invalid length for data (expected {0}, got {0})")]
  InvalidLength(usize, usize),
  #[error("invalid scalar")]
  InvalidScalar,
  #[error("invalid point")]
  InvalidPoint,
}

/// Unified trait to manage a field/group
// This should be moved into its own crate if the need for generic cryptography over ff/group
// continues, which is the exact reason ff/group exists (to provide a generic interface)
// elliptic-curve exists, yet it doesn't really serve the same role, nor does it use &[u8]/Vec<u8>
// It uses GenericArray which will hopefully be deprecated as Rust evolves and doesn't offer enough
// advantages in the modern day to be worth the hassle -- Kayaba
pub trait Curve: Clone + Copy + PartialEq + Eq + Debug {
  /// Field element type
  // This is available via G::Scalar yet `C::G::Scalar` is ambiguous, forcing horrific accesses
  type F: PrimeField;
  /// Group element type
  type G: Group + GroupOps + ScalarMul<Self::F>;
  /// Precomputed table type
  type T: Mul<Self::F, Output = Self::G>;

  /// ID for this curve
  fn id() -> String;
  /// Byte length of the curve ID
  // While curve.id().len() is trivial, this bounds it to u8 and lets us ignore the possibility it
  // contains Unicode, therefore having a String length which is different from its byte length
  fn id_len() -> u8;

  /// Generator for the group
  // While group does provide this in its API, Jubjub users will want to use a custom basepoint
  fn generator() -> Self::G;

  /// Table for the generator for the group
  /// If there isn't a precomputed table available, the generator itself should be used
  fn generator_table() -> Self::T;

  /// Multiexponentation function, presumably Straus or Pippenger
  /// This library does forward an implementation of Straus which should increase key generation
  /// performance by around 4x, also named multiexp_vartime, with a similar API. However, if a more
  /// performant implementation is available, that should be used instead
  // This could also be written as -> Option<C::G> with None for not implemented
  fn multiexp_vartime(scalars: &[Self::F], points: &[Self::G]) -> Self::G;

  /// Hash the message as needed to calculate the binding factor
  /// H3 from the IETF draft
  // This doesn't actually need to be part of Curve as it does nothing with the curve
  // This also solely relates to FROST and with a proper Algorithm/HRAM, all projects using
  // aggregatable signatures over this curve will work without issue, albeit potentially with
  // incompatibilities between FROST implementations
  // It is kept here as Curve + HRAM is effectively a ciphersuite according to the IETF draft
  // and moving it to Schnorr would force all of them into being ciphersuite-specific
  fn hash_msg(msg: &[u8]) -> Vec<u8>;

  /// Field element from hash, used in key generation and to calculate the binding factor
  /// H1 from the IETF draft
  /// Key generation uses it as if it's H2 to generate a challenge for a Proof of Knowledge
  #[allow(non_snake_case)]
  fn hash_to_F(data: &[u8]) -> Self::F;

  // The following methods would optimally be F:: and G:: yet developers can't control F/G
  // They can control a trait they pass into this library

  /// Constant size of a serialized field element
  // The alternative way to grab this would be either serializing a junk element and getting its
  // length or doing a naive division of its BITS property by 8 and assuming a lack of padding
  #[allow(non_snake_case)]
  fn F_len() -> usize;

  /// Constant size of a serialized group element
  // We could grab the serialization as described above yet a naive developer may use a
  // non-constant size encoding, proving yet another reason to force this to be a provided constant
  // A naive developer could still provide a constant for a variable length encoding, yet at least
  // that is on them
  #[allow(non_snake_case)]
  fn G_len() -> usize;

  /// Field element from slice. Preferred to be canonical yet does not have to be
  // Required due to the lack of standardized encoding functions provided by ff/group
  // While they do technically exist, their usage of Self::Repr breaks all potential library usage
  // without helper functions like this
  #[allow(non_snake_case)]
  fn F_from_slice(slice: &[u8]) -> Result<Self::F, CurveError>;

  /// Group element from slice. Must require canonicity or risks differing binding factors
  #[allow(non_snake_case)]
  fn G_from_slice(slice: &[u8]) -> Result<Self::G, CurveError>;

  /// Obtain a vector of the byte encoding of F
  #[allow(non_snake_case)]
  fn F_to_bytes(f: &Self::F) -> Vec<u8>;

  /// Obtain a vector of the byte encoding of G
  #[allow(non_snake_case)]
  fn G_to_bytes(g: &Self::G) -> Vec<u8>;
}

/// Parameters for a multisig
// These fields can not be made public as they should be static
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MultisigParams {
  /// Participants needed to sign on behalf of the group
  t: usize,
  /// Amount of participants
  n: usize,
  /// Index of the participant being acted for
  i: usize,
}

impl MultisigParams {
  pub fn new(
    t: usize,
    n: usize,
    i: usize
  ) -> Result<MultisigParams, FrostError> {
    if (t == 0) || (n == 0) {
      Err(FrostError::ZeroParameter(t, n))?;
    }

    if u16::try_from(n).is_err() {
      Err(FrostError::TooManyParticipants(n, u16::MAX))?;
    }

    // When t == n, this shouldn't be used (MuSig2 and other variants of MuSig exist for a reason),
    // but it's not invalid to do so
    if t > n {
      Err(FrostError::InvalidRequiredQuantity(t, n))?;
    }
    if (i == 0) || (i > n) {
      Err(FrostError::InvalidParticipantIndex(n, i))?;
    }

    Ok(MultisigParams{ t, n, i })
  }

  pub fn t(&self) -> usize { self.t }
  pub fn n(&self) -> usize { self.n }
  pub fn i(&self) -> usize { self.i }
}

#[derive(Error, Debug)]
pub enum FrostError {
  #[error("a parameter was 0 (required {0}, participants {1})")]
  ZeroParameter(usize, usize),
  #[error("too many participants (max {1}, got {0})")]
  TooManyParticipants(usize, u16),
  #[error("invalid amount of required participants (max {1}, got {0})")]
  InvalidRequiredQuantity(usize, usize),
  #[error("invalid participant index (0 < index <= {0}, yet index is {1})")]
  InvalidParticipantIndex(usize, usize),

  #[error("invalid signing set ({0})")]
  InvalidSigningSet(String),
  #[error("invalid participant quantity (expected {0}, got {1})")]
  InvalidParticipantQuantity(usize, usize),
  #[error("duplicated participant index ({0})")]
  DuplicatedIndex(usize),
  #[error("participant 0 provided data despite not existing")]
  NonEmptyParticipantZero,
  #[error("invalid commitment quantity (participant {0}, expected {1}, got {2})")]
  InvalidCommitmentQuantity(usize, usize, usize),
  #[error("invalid commitment (participant {0})")]
  InvalidCommitment(usize),
  #[error("invalid proof of knowledge (participant {0})")]
  InvalidProofOfKnowledge(usize),
  #[error("invalid share (participant {0})")]
  InvalidShare(usize),
  #[error("invalid key generation state machine transition (expected {0}, was {1})")]
  InvalidKeyGenTransition(key_gen::State, key_gen::State),

  #[error("invalid sign state machine transition (expected {0}, was {1})")]
  InvalidSignTransition(sign::State, sign::State),

  #[error("internal error ({0})")]
  InternalError(String),
}

// View of keys passable to algorithm implementations
#[derive(Clone)]
pub struct MultisigView<C: Curve> {
  group_key: C::G,
  included: Vec<usize>,
  secret_share: C::F,
  verification_shares: Vec<C::G>,
}

impl<C: Curve> MultisigView<C> {
  pub fn group_key(&self) -> C::G {
    self.group_key
  }

  pub fn included(&self) -> Vec<usize> {
    self.included.clone()
  }

  pub fn secret_share(&self) -> C::F {
    self.secret_share
  }

  pub fn verification_share(&self, l: usize) -> C::G {
    self.verification_shares[l]
  }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MultisigKeys<C: Curve> {
  /// Multisig Parameters
  params: MultisigParams,

  /// Secret share key
  secret_share: C::F,
  /// Group key
  group_key: C::G,
  /// Verification shares
  verification_shares: Vec<C::G>,

  /// Offset applied to these keys
  offset: Option<C::F>,
}

impl<C: Curve> MultisigKeys<C> {
  pub fn offset(&self, offset: C::F) -> MultisigKeys<C> {
    let mut res = self.clone();
    // Carry any existing offset
    // Enables schemes like Monero's subaddresses which have a per-subaddress offset and then a
    // one-time-key offset
    res.offset = Some(offset + res.offset.unwrap_or(C::F::zero()));
    res
  }

  pub fn params(&self) -> MultisigParams {
    self.params
  }

  pub fn secret_share(&self) -> C::F {
    self.secret_share
  }

  pub fn group_key(&self) -> C::G {
    self.group_key
  }

  pub fn verification_shares(&self) -> Vec<C::G> {
    self.verification_shares.clone()
  }

  pub fn view(&self, included: &[usize]) -> Result<MultisigView<C>, FrostError> {
    if (included.len() < self.params.t) || (self.params.n < included.len()) {
      Err(FrostError::InvalidSigningSet("invalid amount of participants included".to_string()))?;
    }

    let secret_share = self.secret_share * lagrange::<C::F>(self.params.i, &included);
    let offset = self.offset.unwrap_or(C::F::zero());
    let offset_share = offset * C::F::from(included.len().try_into().unwrap()).invert().unwrap();

    Ok(MultisigView {
      group_key: self.group_key + (C::generator_table() * offset),
      secret_share: secret_share + offset_share,
      verification_shares: self.verification_shares.clone().iter().enumerate().map(
        |(l, share)| (*share * lagrange::<C::F>(l, &included)) +
                       (C::generator_table() * offset_share)
      ).collect(),
      included: included.to_vec(),
    })
  }

  pub fn serialized_len(n: usize) -> usize {
    1 + usize::from(C::id_len()) + (3 * 8) + C::F_len() + C::G_len() + (n * C::G_len())
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut serialized = Vec::with_capacity(
      1 + usize::from(C::id_len()) + MultisigKeys::<C>::serialized_len(self.params.n)
    );
    serialized.push(C::id_len());
    serialized.extend(C::id().as_bytes());
    serialized.extend(&(self.params.n as u64).to_le_bytes());
    serialized.extend(&(self.params.t as u64).to_le_bytes());
    serialized.extend(&(self.params.i as u64).to_le_bytes());
    serialized.extend(&C::F_to_bytes(&self.secret_share));
    serialized.extend(&C::G_to_bytes(&self.group_key));
    for i in 1 ..= self.params.n {
      serialized.extend(&C::G_to_bytes(&self.verification_shares[i]));
    }

    serialized
  }

  pub fn deserialize(serialized: &[u8]) -> Result<MultisigKeys<C>, FrostError> {
    if serialized.len() < 1 {
      Err(FrostError::InternalError("MultisigKeys serialization is empty".to_string()))?;
    }

    let id_len: usize = serialized[0].into();
    let mut cursor = 1;

    if serialized.len() < (cursor + id_len) {
      Err(FrostError::InternalError("ID wasn't included".to_string()))?;
    }

    let id = &serialized[cursor .. (cursor + id_len)];
    if C::id().as_bytes() != id {
      Err(
        FrostError::InternalError(
          "curve is distinct between serialization and deserialization".to_string()
        )
      )?;
    }
    cursor += id_len;

    if serialized.len() < (cursor + 8) {
      Err(FrostError::InternalError("participant quantity wasn't included".to_string()))?;
    }

    let n = u64::from_le_bytes(serialized[cursor .. (cursor + 8)].try_into().unwrap()).try_into()
      .map_err(|_| FrostError::InternalError("parameter doesn't fit into usize".to_string()))?;
    cursor += 8;
    if serialized.len() != MultisigKeys::<C>::serialized_len(n) {
      Err(FrostError::InternalError("incorrect serialization length".to_string()))?;
    }

    let t = u64::from_le_bytes(serialized[cursor .. (cursor + 8)].try_into().unwrap()).try_into()
      .map_err(|_| FrostError::InternalError("parameter doesn't fit into usize".to_string()))?;
    cursor += 8;
    let i = u64::from_le_bytes(serialized[cursor .. (cursor + 8)].try_into().unwrap()).try_into()
      .map_err(|_| FrostError::InternalError("parameter doesn't fit into usize".to_string()))?;
    cursor += 8;

    let secret_share = C::F_from_slice(&serialized[cursor .. (cursor + C::F_len())])
      .map_err(|_| FrostError::InternalError("invalid secret share".to_string()))?;
    cursor += C::F_len();
    let group_key = C::G_from_slice(&serialized[cursor .. (cursor + C::G_len())])
      .map_err(|_| FrostError::InternalError("invalid group key".to_string()))?;
    cursor += C::G_len();

    let mut verification_shares = vec![C::G::identity()];
    verification_shares.reserve_exact(n + 1);
    for _ in 0 .. n {
      verification_shares.push(
        C::G_from_slice(&serialized[cursor .. (cursor + C::G_len())])
          .map_err(|_| FrostError::InternalError("invalid verification share".to_string()))?
      );
      cursor += C::G_len();
    }

    Ok(
      MultisigKeys {
        params: MultisigParams::new(t, n, i)
          .map_err(|_| FrostError::InternalError("invalid parameters".to_string()))?,
        secret_share,
        group_key,
        verification_shares,
        offset: None
      }
    )
  }
}
