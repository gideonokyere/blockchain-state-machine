use std::collections::BTreeMap;
use num::traits::{CheckedAdd,CheckedSub,Zero};

pub trait Config:crate::system::Config{
  type Balance:Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T:Config>{
  balances:BTreeMap<T::AccountId,T::Balance>,
}

pub enum Call<T:Config>{
  Transfer{to:T::AccountId,amount:T::Balance}
}

impl<T:Config> Pallet<T>{
  pub fn new()->Self{
    Self { balances: BTreeMap::new() }
  }

  ///Set new balance for the user
  pub fn set_balance(&mut self,who:T::AccountId,amount:T::Balance){
    self.balances.insert(who, amount);
  }

  pub fn balance(&self,who:T::AccountId)->T::Balance{
    *self.balances.get(&who).unwrap_or(&T::Balance::zero())
  }

  ///Tranfer funds from one account to another account
  pub fn transfer(&mut self,caller:T::AccountId,to:T::AccountId,amount:T::Balance)->crate::support::DispatchResult{
    let caller_balance = self.balance(caller.clone());
    let to_balance = self.balance(to.clone());
    let cb = caller_balance.checked_sub(&amount).ok_or("error")?;
    let tc = to_balance.checked_add(&amount).ok_or("overflow occured")?;
    self.set_balance(caller.clone(), cb);
    self.set_balance(to.clone(), tc);
    Ok(())
  }
}

impl<T:Config> crate::support::Dispatch for Pallet<T>{
  type Caller = T::AccountId;
  type Call = Call<T>;
  fn dispatch(&mut self, caller: Self::Caller, pallet_call: Self::Call) -> crate::support::DispatchResult {
      match pallet_call{
        Call::Transfer { to, amount }=>{
          self.transfer(caller, to, amount)?
        }
      }
      Ok(())
  }
}

struct TestConfig;
impl Config for TestConfig{
  type Balance = u128;
}

impl crate::system::Config for TestConfig{
  type AccountId = String;
  type BlockNumber = u32;
  type Nonce = u32;
}

#[test]
fn init_balances(){
  let mut balances: Pallet<TestConfig> = Pallet::new();
  assert_eq!(balances.balance("alice".to_string()),0);
  balances.set_balance("alice".to_string(),100);
  assert_eq!(balances.balance("alice".to_string()),100);
  assert_eq!(balances.balance("bob".to_string()),0);
}

#[test]
fn make_transfer(){
  let mut balances: Pallet<TestConfig> = Pallet::new();
  balances.set_balance("alice".to_string(),100);
  balances.set_balance("bob".to_string(), 0);
  let _ = balances.transfer("alice".to_string(), "bob".to_string(), 20);
  assert_eq!(balances.balance("bob".to_string()),20);
  assert_eq!(balances.balance("alice".to_string()),80);
}