use std::{
  cmp::Ord,
  collections::BTreeMap,
  ops::AddAssign
};
use num::{CheckedAdd,CheckedSub,Zero,One};
// type BlockNumber = u32;
// type AccountId = String;
// //type Nonce = u32;

pub trait Config{
  type AccountId:Ord+Clone;
  type BlockNumber:CheckedAdd + CheckedSub + Zero + One + Copy + AddAssign;
  type Nonce:Zero+Copy+One;
}

#[derive(Debug)]
pub struct Pallet<T:Config>{
  block_number:T::BlockNumber,
  nonce:BTreeMap<T::AccountId,T::BlockNumber>,
}

impl<T:Config> Pallet<T>{
  pub fn new()->Self{
    Self{block_number:T::BlockNumber::zero(),nonce:BTreeMap::new()}
  }

  ///Get the current block number
  pub fn block_number(&self)->T::BlockNumber{
    self.block_number
  }

  ///This function can be use to increases the block number
  /// Increases the block number by one
  pub fn inc_block_number(&mut self){
    self.block_number+=T::BlockNumber::one();
  }

  ///inc the nonce of an account
  pub fn inc_nonce(&mut self,who:&T::AccountId){
    let zero = T::BlockNumber::zero();
    let account_nonce = self.nonce.get(&who).unwrap_or(&zero);
    let new_nonce = account_nonce
        .checked_add(&T::BlockNumber::one())
        .ok_or("error");
    self.nonce.insert(who.clone(), new_nonce.unwrap());
  }
}

#[cfg(test)]
mod test{

  struct TestConfig;

  impl super::Config for TestConfig{
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
  }

  #[test]
  fn init_system(){
    let mut block:super::Pallet<TestConfig> = super::Pallet::new();
    block.inc_block_number();
    assert_eq!(block.block_number(),1);
    block.inc_nonce(&"alice".to_string());
    assert_eq!(block.nonce.get(&"alice".to_string()),Some(&1));
  }
}