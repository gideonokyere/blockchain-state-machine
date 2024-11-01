mod balances;
mod system;
mod support;
mod proof_of_existence;
use std::vec;

use crate::support::Dispatch;

mod types {
	pub type AccountId = String;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId,crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header,Extrinsic>;
}

pub enum RuntimeCall{
 Balances(balances::Call<Runtime>),
 Poe(proof_of_existence::Call<Runtime>),
}

impl system::Config for Runtime{
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime{
    type Balance = u128;
}

impl proof_of_existence::Config for Runtime{
    type Content = String;
}

#[derive(Debug)]
pub struct Runtime{
    balances:balances::Pallet<Runtime>,
    system:system::Pallet<Runtime>,
    poe:proof_of_existence::Pallet<Runtime>,
}

impl Runtime {
    fn new()->Self{
        Self{
            balances:balances::Pallet::new(),
            system:system::Pallet::new(),
            poe:proof_of_existence::Pallet::new(),
        }
    }

    fn execute_block(&mut self,block:types::Block)->support::DispatchResult{
        self.system.inc_block_number();
        assert_eq!(self.system.block_number(),block.header.block_number);
        for (i,support::Extrinsic{caller,call}) in block.extrinsics.into_iter().enumerate(){
            self.system.inc_nonce(&caller);
            let _res = self.dispatch(caller, call).map_err(|e|println!(
                "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
            ));
        }
        Ok(())
    }
}

impl crate::support::Dispatch for Runtime{
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller,runtime_call: Self::Call) -> support::DispatchResult {
        match runtime_call{
            RuntimeCall::Balances(call)=>{
                self.balances.dispatch(caller, call)?
            },
            RuntimeCall::Poe(call)=>{
                self.poe.dispatch(caller, call)?
            }
        }
        Ok(())
    }
}

fn main() {
   let mut runtime = Runtime::new();
   //First set Alice balance to 100
   runtime.balances.set_balance("alice".to_string(), 100);

    let block_1 = types::Block{
        header:support::Header { block_number: 1 },
        extrinsics:vec![
            support::Extrinsic{
                caller:"alice".to_string(),
                call:RuntimeCall::Balances(balances::Call::Transfer { to: "bob".to_string(), amount: 20 }),
            },
            support::Extrinsic{
                caller:"alice".to_string(),
                call:RuntimeCall::Balances(balances::Call::Transfer{ to: "Charlie".to_string(), amount: 10 })
            }
        ]
    };

    let block_2 = types::Block{
        header:support::Header { block_number: 2 },
        extrinsics:vec![
            support::Extrinsic{
                caller:"bob".to_string(),
                call:RuntimeCall::Poe(proof_of_existence::Call::CreateClaim { claim: "Blue is london".to_string() })
            }
        ]
    };

    let block_3 = types::Block{
        header:support::Header { block_number: 3 },
        extrinsics:vec![
            support::Extrinsic{
                caller:"charlie".to_string(),
                call:RuntimeCall::Poe(proof_of_existence::Call::RevokeClaim { cliam: "Blue is london".to_string() })
            }
        ]
    };

    runtime.execute_block(block_1).expect("Invalid Block");
    runtime.execute_block(block_2).expect("Invalid Block");
    runtime.execute_block(block_3).expect("Invalid Block");

   println!("{:#?}",runtime);
}
