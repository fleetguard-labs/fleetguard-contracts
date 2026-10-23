#![no_std]
#[cfg(test)]
extern crate std;
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, String};
#[cfg(not(feature="instance"))]
use soroban_sdk::{ContractExecutable, ContractExecutableRef};
#[contracttype]
#[derive(Clone)]
enum Key { Admin, Delay, Pending, Value }
#[contracttype]
#[derive(Clone)]
pub struct Proposal { pub tag: String, pub expected: BytesN<32>, pub next: BytesN<32>, pub ready: u32 }
#[contract]
pub struct FleetGuard;
#[cfg(not(feature="instance"))]
fn admin(env:&Env) { let a:Address=env.storage().instance().get(&Key::Admin).unwrap(); a.require_auth(); }
#[cfg(not(feature="instance"))]
#[contractimpl]
impl FleetGuard {
 pub fn __constructor(env:Env, administrator:Address, delay:u32) { assert!(delay>=2 && delay<=100000,"delay bounds"); env.storage().instance().set(&Key::Admin,&administrator);env.storage().instance().set(&Key::Delay,&delay); }
 pub fn publish(env:Env, tag:String, hash:BytesN<32>) {admin(&env);assert!(!env.executable_refs().has(&tag),"already published");env.executable_refs().set(&tag,&hash);}
 pub fn current(env:Env,tag:String)->BytesN<32>{env.executable_refs().get(&tag).unwrap()}
 pub fn spawn(env:Env,tag:String,salt:BytesN<32>,value:i128)->Address{admin(&env);let a:Address=env.storage().instance().get(&Key::Admin).unwrap();env.deployer().with_current_contract(salt).deploy_contract(ContractExecutable::ExternalRef(ContractExecutableRef{owner:env.current_contract_address(),tag}),(a,value))}
 pub fn propose(env:Env,tag:String,expected:BytesN<32>,next:BytesN<32>)->u32{admin(&env);assert!(!env.storage().instance().has(&Key::Pending),"pending proposal");assert!(env.executable_refs().get(&tag)==Some(expected.clone()),"stale hash");assert!(expected!=next,"unchanged hash");let delay:u32=env.storage().instance().get(&Key::Delay).unwrap();let ready=env.ledger().sequence().checked_add(delay).unwrap();env.storage().instance().set(&Key::Pending,&Proposal{tag,expected,next,ready});ready}
 pub fn execute(env:Env)->BytesN<32>{admin(&env);let p:Proposal=env.storage().instance().get(&Key::Pending).unwrap();assert!(env.ledger().sequence()>=p.ready,"timelock");assert!(env.executable_refs().get(&p.tag)==Some(p.expected),"stale hash");env.executable_refs().set(&p.tag,&p.next);env.storage().instance().remove(&Key::Pending);p.next}
 pub fn cancel(env:Env){admin(&env);env.storage().instance().remove(&Key::Pending);}
 pub fn pending(env:Env)->Option<Proposal>{env.storage().instance().get(&Key::Pending)}
}
#[cfg(feature="instance")]
#[contractimpl]
impl FleetGuard{
 pub fn __constructor(env:Env,administrator:Address,value:i128){env.storage().instance().set(&Key::Admin,&administrator);env.storage().instance().set(&Key::Value,&value);}
 pub fn value(env:Env)->i128{env.storage().instance().get(&Key::Value).unwrap()}
 pub fn add(env:Env,amount:i128)->i128{let a:Address=env.storage().instance().get(&Key::Admin).unwrap();a.require_auth();let v:i128=env.storage().instance().get(&Key::Value).unwrap();let next=v.checked_add(amount).unwrap();env.storage().instance().set(&Key::Value,&next);next}
 pub fn version()->u32{if cfg!(feature="v2"){2}else{1}}
}
#[cfg(all(test,not(feature="instance")))]
mod tests{
 use super::*;use soroban_sdk::testutils::{Address as _,Ledger};
 fn setup()->(Env,Address,Address){let e=Env::default();e.mock_all_auths();let a=Address::generate(&e);let id=e.register(FleetGuard,(&a,2u32));(e,id,a)}
 #[test]fn constructor_rejects_zero_delay(){let e=Env::default();let a=Address::generate(&e);assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||e.register(FleetGuard,(&a,0u32)))).is_err());}
 #[test]fn unsigned_admin_call_is_rejected(){let e=Env::default();let a=Address::generate(&e);let id=e.register(FleetGuard,(&a,2u32));let c=FleetGuardClient::new(&e,&id);assert!(c.try_cancel().is_err());}
 #[test]fn pending_initially_empty_and_cancel_is_idempotent(){let(e,id,_)=setup();let c=FleetGuardClient::new(&e,&id);assert!(c.pending().is_none());c.cancel();assert!(c.pending().is_none());}
 #[test]fn upgrade_requires_delay_and_preserves_state(){let(e,id,a)=setup();let c=FleetGuardClient::new(&e,&id);let h1=e.deployer().upload_contract_wasm(&include_bytes!("v1.wasm")[..]);let h2=e.deployer().upload_contract_wasm(&include_bytes!("v2.wasm")[..]);let tag=String::from_str(&e,"stable");c.publish(&tag,&h1);let x=c.spawn(&tag,&BytesN::from_array(&e,&[1;32]),&41i128);let version=soroban_sdk::Symbol::new(&e,"version");let value=soroban_sdk::Symbol::new(&e,"value");assert_eq!(e.invoke_contract::<u32>(&x,&version,soroban_sdk::vec![&e]),1);assert_eq!(e.invoke_contract::<i128>(&x,&value,soroban_sdk::vec![&e]),41);assert!(c.try_publish(&tag,&h2).is_err());assert!(c.try_propose(&tag,&h2,&h1).is_err());let ready=c.propose(&tag,&h1,&h2);assert!(c.try_propose(&tag,&h1,&h2).is_err());assert!(c.try_execute().is_err());e.ledger().with_mut(|l|l.sequence_number=ready);assert_eq!(c.execute(),h2);assert_eq!(e.invoke_contract::<u32>(&x,&version,soroban_sdk::vec![&e]),2);assert_eq!(e.invoke_contract::<i128>(&x,&value,soroban_sdk::vec![&e]),41);assert!(c.pending().is_none());let _=a;}
}
