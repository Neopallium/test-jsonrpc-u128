use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

use serde::{Deserialize, Serialize};

pub type Balance = u128;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountKey([u8; 32]);

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthData {
    SetPayingKey(AccountKey, AccountKey, Balance),
    NoData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    from: AccountKey,
    data: AuthData,
}

#[rpc]
pub trait Rpc {
  /// Test that returns type with `u128`
  #[rpc(name = "get_auth")]
  fn get_auth(&self) -> Result<Auth>;
}

pub struct RpcImpl;
impl Rpc for RpcImpl {
  fn get_auth(&self) -> Result<Auth> {
    let val = Auth {
        from: AccountKey([1u8; 32]),
        data: AuthData::SetPayingKey(AccountKey([2u8; 32]), AccountKey([3u8; 32]), 12345678901234u128),
    };
    eprintln!("val = {:?}", val);
    Ok(val)
  }
}

/*
fn test_json() -> Result<()> {
    let val = Auth {
        from: AccountKey([1u8; 32]),
        data: AuthData::SetPayingKey(AccountKey([2u8; 32]), AccountKey([3u8; 32]), 12345678901234u128),
    };
    eprintln!("val = {:?}", val);
    eprintln!("json = {}", serde_json::to_string(&val)?);
    Ok(())
}
*/

fn main() {
  let mut io = IoHandler::default();
  io.extend_with(RpcImpl.to_delegate());

  let server = ServerBuilder::new(io)
    .threads(3)
    .start_http(&"127.0.0.1:3030".parse().unwrap())
    .unwrap();

  server.wait();
}

