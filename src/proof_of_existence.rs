use core::fmt::Debug;
use std::collections::BTreeMap;
use crate::support::DispatchResult;
pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

pub enum Call<T:Config>{
  CreateClaim{claim:T::Content},
  RevokeClaim{cliam:T::Content},
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
  pub claims:BTreeMap<T::Content,T::AccountId>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self { claims:BTreeMap::new() }
	}

  /// Get the owner if any of a claim
  pub fn get_claim(&self,claim:&T::Content)->Option<&T::AccountId>{
    self.claims.get(&claim)
  }

  /// Create new Cliam
  pub fn create_claim(&mut self,caller:T::AccountId,claim:T::Content)->DispatchResult{
    if self.claims.contains_key(&claim){
      return Err(&"A claim has already been made");
    }
    self.claims.insert(claim, caller);
    Ok(())
  }

  /// Revoke Claim
  pub fn revoke_claim(&mut self,caller:T::AccountId,claim:T::Content)->DispatchResult{
    let owner = self.get_claim(&claim).ok_or("File does not exsit")?;
    if caller != *owner {
      return Err(&"You are not the rightful owner");
    }
    self.claims.remove(&claim);
      Ok(())
}
}

impl<T:Config> crate::support::Dispatch for Pallet<T>{
  type Caller = T::AccountId;
  type Call = Call<T>;
  
  fn dispatch(&mut self, caller: Self::Caller, pallet_call: Self::Call) ->crate::support::DispatchResult {
      match pallet_call{
        Call::CreateClaim { claim}=>{
          self.create_claim(caller, claim)?;
        },
        Call::RevokeClaim { cliam }=>{
          self.revoke_claim(caller,cliam)?;
        }
      }
      Ok(())
  }
}

#[cfg(test)]
mod test{

  struct TestConfig;
  impl super::Config for TestConfig{
    type Content = String;
  }
  impl crate::system::Config for TestConfig{
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
  }
  #[test]
  fn basic_proof_of_existence(){
    let mut pallet:super::Pallet<TestConfig> = super::Pallet::new();
    assert_eq!(pallet.get_claim(&"Blue is london".to_string()),None);
    let _res = pallet.create_claim("alice".to_string(), "Blue is london".to_string());
    assert_eq!(pallet.get_claim(&"Blue is london".to_string()),Some("alice".to_string()).as_ref());
    //Bob trying to revoke a cliam that does not belongs to him
    assert_eq!(pallet.revoke_claim("bob".to_string(), "Blue is london".to_string()),Err("You are not the rightful owner"));
    // Alice should be able to revoke her cliam
    let _ = pallet.revoke_claim("alice".to_string(), "Blue is london".to_string());
    assert_eq!(pallet.get_claim(&"Blue is london".to_string()),None);
  }
}