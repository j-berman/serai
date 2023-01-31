pub use router::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod router {
  #![allow(clippy::enum_variant_names)]
  #![allow(dead_code)]
  #![allow(clippy::type_complexity)]
  #![allow(unused_imports)]
  #[doc = "Router was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
  use std::sync::Arc;
  use ethers::core::{
    abi::{Abi, Token, Detokenize, InvalidOutputType, Tokenizable},
    types::*,
  };
  use ethers::contract::{
    Contract,
    builders::{ContractCall, Event},
    Lazy,
  };
  use ethers::providers::Middleware;
  # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PublicKeyAlreadySet\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Unauthorized\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"VerificationError\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"success\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Executed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"Q\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Router.RTransaction[]\",\"name\":\"transactions\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct Router.RSignature\",\"name\":\"sig\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"e\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"execute\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"publicKey\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"parity\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"px\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Router.RPublicKey\",\"name\":\"_publicKey\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"parity\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"px\",\"type\":\"bytes32\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPublicKey\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct Router.RPublicKey\",\"name\":\"_publicKey\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint8\",\"name\":\"parity\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"px\",\"type\":\"bytes32\",\"components\":[]}]},{\"internalType\":\"struct Router.RSignature\",\"name\":\"sig\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"e\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePublicKey\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"parity\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"px\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"message\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"e\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verify\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]" ;
  #[doc = r" The parsed JSON-ABI of the contract."]
  pub static ROUTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
    ethers::contract::Lazy::new(|| {
      ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
    });
  #[doc = r" Bytecode of the #name contract"]
  pub static ROUTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
    ethers::contract::Lazy::new(|| {
      "0x608060405234801561001057600080fd5b50600160008190555033600160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506114c6806100696000396000f3fe608060405234801561001057600080fd5b50600436106100885760003560e01c80638da5cb5b1161005b5780638da5cb5b146101145780639186da4c14610132578063affed0e014610162578063e493ef8c1461018057610088565b80631811024d1461008d57806337088665146100bd578063459e93e5146100d957806363ffab31146100f5575b600080fd5b6100a760048036038101906100a29190610a05565b61019e565b6040516100b49190610a80565b60405180910390f35b6100d760048036038101906100d29190610b24565b610400565b005b6100f360048036038101906100ee9190610b64565b6104cd565b005b6100fd6105c8565b60405161010b929190610baf565b60405180910390f35b61011c6105e7565b6040516101299190610c19565b60405180910390f35b61014c60048036038101906101479190610c34565b61060d565b6040516101599190610a80565b60405180910390f35b61016a6107f2565b6040516101779190610cc8565b60405180910390f35b6101886107f8565b6040516101959190610cc8565b60405180910390f35b60006101a861081c565b600060025485856040516020016101c193929190610fa3565b604051602081830303815290604052805190602001209050610205600360000160009054906101000a900460ff16600360010154838660000151876020015161060d565b61023b576040517ffbcb0b3400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600080600090505b8686905081101561039457600087878381811061026357610262610fd5565b5b90506020028101906102759190611013565b6000016020810190610287919061103b565b73ffffffffffffffffffffffffffffffffffffffff168888848181106102b0576102af610fd5565b5b90506020028101906102c29190611013565b602001358989858181106102d9576102d8610fd5565b5b90506020028101906102eb9190611013565b60400135908a8a8681811061030357610302610fd5565b5b90506020028101906103159190611013565b80606001906103249190611068565b6040516103329291906110fb565b600060405180830381858888f193505050503d8060008114610370576040519150601f19603f3d011682016040523d82523d6000602084013e610375565b606091505b5050905080821b8317925050808061038c90611143565b915050610243565b507f6cbfbb9b98ba7bb20bf4e76a5755fce5428cbeb7fdd7cd433fd3d63062476b80600254826040516103c892919061118b565b60405180910390a1600260008154809291906103e390611143565b91905055506000811415925050506103f961086b565b9392505050565b60008260000151836020015160405160200161041d92919061120b565b604051602081830303815290604052805190602001209050610461600360000160009054906101000a900460ff16600360010154838560000151866020015161060d565b610497576040517ffbcb0b3400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b82600360008201518160000160006101000a81548160ff021916908360ff16021790555060208201518160010155905050505050565b600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610554576040517f82b4290000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000801b60036001015414610595576040517f3b3cf97000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8060000151600360000160006101000a81548160ff021916908360ff160217905550806020015160036001018190555050565b60038060000160009054906101000a900460ff16908060010154905082565b600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b6000807ffffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd03641418061063f5761063e611237565b5b8660001c8460001c097ffffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd03641416106739190611266565b60001b905060007ffffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141806106a9576106a8611237565b5b8760001c8660001c097ffffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd03641416106dd9190611266565b60001b90506000801b82036106f157600080fd5b60006001838a8a8560405160008152602001604052604051610716949392919061129a565b6020604051602081039080840390855afa158015610738573d6000803e3d6000fd5b505050602060405103519050600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036107b3576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016107aa9061133c565b60405180910390fd5b808989468a6040516020016107cc9594939291906113c5565b604051602081830303815290604052805190602001208614935050505095945050505050565b60025481565b7ffffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd036414181565b600260005403610861576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161085890611470565b60405180910390fd5b6002600081905550565b6001600081905550565b6000604051905090565b600080fd5b600080fd5b600080fd5b600080fd5b600080fd5b60008083601f8401126108ae576108ad610889565b5b8235905067ffffffffffffffff8111156108cb576108ca61088e565b5b6020830191508360208202830111156108e7576108e6610893565b5b9250929050565b600080fd5b6000601f19601f8301169050919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b61093c826108f3565b810181811067ffffffffffffffff8211171561095b5761095a610904565b5b80604052505050565b600061096e610875565b905061097a8282610933565b919050565b6000819050919050565b6109928161097f565b811461099d57600080fd5b50565b6000813590506109af81610989565b92915050565b6000604082840312156109cb576109ca6108ee565b5b6109d56040610964565b905060006109e5848285016109a0565b60008301525060206109f9848285016109a0565b60208301525092915050565b600080600060608486031215610a1e57610a1d61087f565b5b600084013567ffffffffffffffff811115610a3c57610a3b610884565b5b610a4886828701610898565b93509350506020610a5b868287016109b5565b9150509250925092565b60008115159050919050565b610a7a81610a65565b82525050565b6000602082019050610a956000830184610a71565b92915050565b600060ff82169050919050565b610ab181610a9b565b8114610abc57600080fd5b50565b600081359050610ace81610aa8565b92915050565b600060408284031215610aea57610ae96108ee565b5b610af46040610964565b90506000610b0484828501610abf565b6000830152506020610b18848285016109a0565b60208301525092915050565b60008060808385031215610b3b57610b3a61087f565b5b6000610b4985828601610ad4565b9250506040610b5a858286016109b5565b9150509250929050565b600060408284031215610b7a57610b7961087f565b5b6000610b8884828501610ad4565b91505092915050565b610b9a81610a9b565b82525050565b610ba98161097f565b82525050565b6000604082019050610bc46000830185610b91565b610bd16020830184610ba0565b9392505050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b6000610c0382610bd8565b9050919050565b610c1381610bf8565b82525050565b6000602082019050610c2e6000830184610c0a565b92915050565b600080600080600060a08688031215610c5057610c4f61087f565b5b6000610c5e88828901610abf565b9550506020610c6f888289016109a0565b9450506040610c80888289016109a0565b9350506060610c91888289016109a0565b9250506080610ca2888289016109a0565b9150509295509295909350565b6000819050919050565b610cc281610caf565b82525050565b6000602082019050610cdd6000830184610cb9565b92915050565b600082825260208201905092915050565b6000819050919050565b610d0781610bf8565b8114610d1257600080fd5b50565b600081359050610d2481610cfe565b92915050565b6000610d396020840184610d15565b905092915050565b610d4a81610bf8565b82525050565b610d5981610caf565b8114610d6457600080fd5b50565b600081359050610d7681610d50565b92915050565b6000610d8b6020840184610d67565b905092915050565b610d9c81610caf565b82525050565b600080fd5b600080fd5b600080fd5b60008083356001602003843603038112610dce57610dcd610dac565b5b83810192508235915060208301925067ffffffffffffffff821115610df657610df5610da2565b5b600182023603831315610e0c57610e0b610da7565b5b509250929050565b600082825260208201905092915050565b82818337600083830152505050565b6000610e408385610e14565b9350610e4d838584610e25565b610e56836108f3565b840190509392505050565b600060808301610e746000840184610d2a565b610e816000860182610d41565b50610e8f6020840184610d7c565b610e9c6020860182610d93565b50610eaa6040840184610d7c565b610eb76040860182610d93565b50610ec56060840184610db1565b8583036060870152610ed8838284610e34565b925050508091505092915050565b6000610ef28383610e61565b905092915050565b600082356001608003833603038112610f1657610f15610dac565b5b82810191505092915050565b6000602082019050919050565b6000610f3b8385610ce3565b935083602084028501610f4d84610cf4565b8060005b87811015610f91578484038952610f688284610efa565b610f728582610ee6565b9450610f7d83610f22565b925060208a01995050600181019050610f51565b50829750879450505050509392505050565b6000604082019050610fb86000830186610cb9565b8181036020830152610fcb818486610f2f565b9050949350505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b600080fd5b600080fd5b600080fd5b60008235600160800383360303811261102f5761102e611004565b5b80830191505092915050565b6000602082840312156110515761105061087f565b5b600061105f84828501610d15565b91505092915050565b6000808335600160200384360303811261108557611084611004565b5b80840192508235915067ffffffffffffffff8211156110a7576110a6611009565b5b6020830192506001820236038313156110c3576110c261100e565b5b509250929050565b600081905092915050565b60006110e283856110cb565b93506110ef838584610e25565b82840190509392505050565b60006111088284866110d6565b91508190509392505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b600061114e82610caf565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036111805761117f611114565b5b600182019050919050565b60006040820190506111a06000830185610cb9565b6111ad6020830184610cb9565b9392505050565b60008160f81b9050919050565b60006111cc826111b4565b9050919050565b6111e46111df82610a9b565b6111c1565b82525050565b6000819050919050565b6112056112008261097f565b6111ea565b82525050565b600061121782856111d3565b60018201915061122782846111f4565b6020820191508190509392505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b600061127182610caf565b915061127c83610caf565b925082820390508181111561129457611293611114565b5b92915050565b60006080820190506112af6000830187610ba0565b6112bc6020830186610b91565b6112c96040830185610ba0565b6112d66060830184610ba0565b95945050505050565b600082825260208201905092915050565b7f65637265636f766572206661696c656400000000000000000000000000000000600082015250565b60006113266010836112df565b9150611331826112f0565b602082019050919050565b6000602082019050818103600083015261135581611319565b9050919050565b60008160601b9050919050565b60006113748261135c565b9050919050565b600061138682611369565b9050919050565b61139e61139982610bf8565b61137b565b82525050565b6000819050919050565b6113bf6113ba82610caf565b6113a4565b82525050565b60006113d1828861138d565b6014820191506113e182876111d3565b6001820191506113f182866111f4565b60208201915061140182856113ae565b60208201915061141182846111f4565b6020820191508190509695505050505050565b7f5265656e7472616e637947756172643a207265656e7472616e742063616c6c00600082015250565b600061145a601f836112df565b915061146582611424565b602082019050919050565b600060208201905081810360008301526114898161144d565b905091905056fea2646970667358221220b6bcbb87dfb9898acabfe5b12de051cedb11e52adaac799b934a115b66554c2d64736f6c63430008110033" . parse () . expect ("invalid bytecode")
    });
  pub struct Router<M>(ethers::contract::Contract<M>);
  impl<M> Clone for Router<M> {
    fn clone(&self) -> Self {
      Router(self.0.clone())
    }
  }
  impl<M> std::ops::Deref for Router<M> {
    type Target = ethers::contract::Contract<M>;
    fn deref(&self) -> &Self::Target {
      &self.0
    }
  }
  impl<M> std::fmt::Debug for Router<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      f.debug_tuple(stringify!(Router)).field(&self.address()).finish()
    }
  }
  impl<M: ethers::providers::Middleware> Router<M> {
    #[doc = r" Creates a new contract instance with the specified `ethers`"]
    #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
    #[doc = r" object"]
    pub fn new<T: Into<ethers::core::types::Address>>(
      address: T,
      client: ::std::sync::Arc<M>,
    ) -> Self {
      ethers::contract::Contract::new(address.into(), ROUTER_ABI.clone(), client).into()
    }
    #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
    #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
    #[doc = r""]
    #[doc = r" Notes:"]
    #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
    #[doc = r" 1. The default poll duration is 7 seconds."]
    #[doc = r" 1. The default number of confirmations is 1 block."]
    #[doc = r""]
    #[doc = r""]
    #[doc = r" # Example"]
    #[doc = r""]
    #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
    #[doc = r""]
    #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
    #[doc = r""]
    #[doc = r" ```ignore"]
    #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
    #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
    #[doc = r""]
    #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
    #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
    #[doc = r" # }"]
    #[doc = r" ```"]
    pub fn deploy<T: ethers::core::abi::Tokenize>(
      client: ::std::sync::Arc<M>,
      constructor_args: T,
    ) -> ::std::result::Result<
      ethers::contract::builders::ContractDeployer<M, Self>,
      ethers::contract::ContractError<M>,
    > {
      let factory = ethers::contract::ContractFactory::new(
        ROUTER_ABI.clone(),
        ROUTER_BYTECODE.clone().into(),
        client,
      );
      let deployer = factory.deploy(constructor_args)?;
      let deployer = ethers::contract::ContractDeployer::new(deployer);
      Ok(deployer)
    }
    #[doc = "Calls the contract's `Q` (0xe493ef8c) function"]
    pub fn q(&self) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
      self
        .0
        .method_hash([228, 147, 239, 140], ())
        .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `execute` (0x1811024d) function"]
    pub fn execute(
      &self,
      transactions: ::std::vec::Vec<Rtransaction>,
      sig: Rsignature,
    ) -> ethers::contract::builders::ContractCall<M, bool> {
      self
        .0
        .method_hash([24, 17, 2, 77], (transactions, sig))
        .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `nonce` (0xaffed0e0) function"]
    pub fn nonce(&self) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
      self
        .0
        .method_hash([175, 254, 208, 224], ())
        .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
    pub fn owner(
      &self,
    ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
      self
        .0
        .method_hash([141, 165, 203, 91], ())
        .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `publicKey` (0x63ffab31) function"]
    pub fn public_key(&self) -> ethers::contract::builders::ContractCall<M, (u8, [u8; 32])> {
      self
        .0
        .method_hash([99, 255, 171, 49], ())
        .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `setPublicKey` (0x459e93e5) function"]
    pub fn set_public_key(
      &self,
      public_key: RpublicKey,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
      self
        .0
        .method_hash([69, 158, 147, 229], (public_key,))
        .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `updatePublicKey` (0x37088665) function"]
    pub fn update_public_key(
      &self,
      public_key: RpublicKey,
      sig: Rsignature,
    ) -> ethers::contract::builders::ContractCall<M, ()> {
      self
        .0
        .method_hash([55, 8, 134, 101], (public_key, sig))
        .expect("method not found (this should never happen)")
    }
    #[doc = "Calls the contract's `verify` (0x9186da4c) function"]
    pub fn verify(
      &self,
      parity: u8,
      px: [u8; 32],
      message: [u8; 32],
      e: [u8; 32],
      s: [u8; 32],
    ) -> ethers::contract::builders::ContractCall<M, bool> {
      self
        .0
        .method_hash([145, 134, 218, 76], (parity, px, message, e, s))
        .expect("method not found (this should never happen)")
    }
    #[doc = "Gets the contract's `Executed` event"]
    pub fn executed_filter(&self) -> ethers::contract::builders::Event<M, ExecutedFilter> {
      self.0.event()
    }
    #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
    pub fn events(&self) -> ethers::contract::builders::Event<M, ExecutedFilter> {
      self.0.event_with_filter(Default::default())
    }
  }
  impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Router<M> {
    fn from(contract: ethers::contract::Contract<M>) -> Self {
      Self(contract)
    }
  }
  #[doc = "Custom Error type `PublicKeyAlreadySet` with signature `PublicKeyAlreadySet()` and selector `[59, 60, 249, 112]`"]
  #[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthError,
    ethers :: contract :: EthDisplay,
  )]
  #[etherror(name = "PublicKeyAlreadySet", abi = "PublicKeyAlreadySet()")]
  pub struct PublicKeyAlreadySet;
  #[doc = "Custom Error type `Unauthorized` with signature `Unauthorized()` and selector `[130, 180, 41, 0]`"]
  #[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthError,
    ethers :: contract :: EthDisplay,
  )]
  #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
  pub struct Unauthorized;
  #[doc = "Custom Error type `VerificationError` with signature `VerificationError()` and selector `[251, 203, 11, 52]`"]
  #[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthError,
    ethers :: contract :: EthDisplay,
  )]
  #[etherror(name = "VerificationError", abi = "VerificationError()")]
  pub struct VerificationError;
  #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
  pub enum RouterErrors {
    PublicKeyAlreadySet(PublicKeyAlreadySet),
    Unauthorized(Unauthorized),
    VerificationError(VerificationError),
  }
  impl ethers::core::abi::AbiDecode for RouterErrors {
    fn decode(data: impl AsRef<[u8]>) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
      if let Ok(decoded) =
        <PublicKeyAlreadySet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
      {
        return Ok(RouterErrors::PublicKeyAlreadySet(decoded));
      }
      if let Ok(decoded) = <Unauthorized as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
        return Ok(RouterErrors::Unauthorized(decoded));
      }
      if let Ok(decoded) =
        <VerificationError as ethers::core::abi::AbiDecode>::decode(data.as_ref())
      {
        return Ok(RouterErrors::VerificationError(decoded));
      }
      Err(ethers::core::abi::Error::InvalidData.into())
    }
  }
  impl ethers::core::abi::AbiEncode for RouterErrors {
    fn encode(self) -> Vec<u8> {
      match self {
        RouterErrors::PublicKeyAlreadySet(element) => element.encode(),
        RouterErrors::Unauthorized(element) => element.encode(),
        RouterErrors::VerificationError(element) => element.encode(),
      }
    }
  }
  impl ::std::fmt::Display for RouterErrors {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
      match self {
        RouterErrors::PublicKeyAlreadySet(element) => element.fmt(f),
        RouterErrors::Unauthorized(element) => element.fmt(f),
        RouterErrors::VerificationError(element) => element.fmt(f),
      }
    }
  }
  impl ::std::convert::From<PublicKeyAlreadySet> for RouterErrors {
    fn from(var: PublicKeyAlreadySet) -> Self {
      RouterErrors::PublicKeyAlreadySet(var)
    }
  }
  impl ::std::convert::From<Unauthorized> for RouterErrors {
    fn from(var: Unauthorized) -> Self {
      RouterErrors::Unauthorized(var)
    }
  }
  impl ::std::convert::From<VerificationError> for RouterErrors {
    fn from(var: VerificationError) -> Self {
      RouterErrors::VerificationError(var)
    }
  }
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthEvent,
    ethers :: contract :: EthDisplay,
    Default,
  )]
  #[ethevent(name = "Executed", abi = "Executed(uint256,uint256)")]
  pub struct ExecutedFilter {
    pub nonce: ethers::core::types::U256,
    pub success: ethers::core::types::U256,
  }
  #[doc = "Container type for all input parameters for the `Q` function with signature `Q()` and selector `[228, 147, 239, 140]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
  )]
  #[ethcall(name = "Q", abi = "Q()")]
  pub struct QCall;
  #[doc = "Container type for all input parameters for the `execute` function with signature `execute((address,uint256,uint256,bytes)[],(bytes32,bytes32))` and selector `[24, 17, 2, 77]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
  )]
  #[ethcall(name = "execute", abi = "execute((address,uint256,uint256,bytes)[],(bytes32,bytes32))")]
  pub struct ExecuteCall {
    pub transactions: ::std::vec::Vec<Rtransaction>,
    pub sig: Rsignature,
  }
  #[doc = "Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `[175, 254, 208, 224]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
  )]
  #[ethcall(name = "nonce", abi = "nonce()")]
  pub struct NonceCall;
  #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
  )]
  #[ethcall(name = "owner", abi = "owner()")]
  pub struct OwnerCall;
  #[doc = "Container type for all input parameters for the `publicKey` function with signature `publicKey()` and selector `[99, 255, 171, 49]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
  )]
  #[ethcall(name = "publicKey", abi = "publicKey()")]
  pub struct PublicKeyCall;
  #[doc = "Container type for all input parameters for the `setPublicKey` function with signature `setPublicKey((uint8,bytes32))` and selector `[69, 158, 147, 229]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
  )]
  #[ethcall(name = "setPublicKey", abi = "setPublicKey((uint8,bytes32))")]
  pub struct SetPublicKeyCall {
    pub public_key: RpublicKey,
  }
  #[doc = "Container type for all input parameters for the `updatePublicKey` function with signature `updatePublicKey((uint8,bytes32),(bytes32,bytes32))` and selector `[55, 8, 134, 101]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
  )]
  #[ethcall(name = "updatePublicKey", abi = "updatePublicKey((uint8,bytes32),(bytes32,bytes32))")]
  pub struct UpdatePublicKeyCall {
    pub public_key: RpublicKey,
    pub sig: Rsignature,
  }
  #[doc = "Container type for all input parameters for the `verify` function with signature `verify(uint8,bytes32,bytes32,bytes32,bytes32)` and selector `[145, 134, 218, 76]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthCall,
    ethers :: contract :: EthDisplay,
    Default,
  )]
  #[ethcall(name = "verify", abi = "verify(uint8,bytes32,bytes32,bytes32,bytes32)")]
  pub struct VerifyCall {
    pub parity: u8,
    pub px: [u8; 32],
    pub message: [u8; 32],
    pub e: [u8; 32],
    pub s: [u8; 32],
  }
  #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
  pub enum RouterCalls {
    Q(QCall),
    Execute(ExecuteCall),
    Nonce(NonceCall),
    Owner(OwnerCall),
    PublicKey(PublicKeyCall),
    SetPublicKey(SetPublicKeyCall),
    UpdatePublicKey(UpdatePublicKeyCall),
    Verify(VerifyCall),
  }
  impl ethers::core::abi::AbiDecode for RouterCalls {
    fn decode(data: impl AsRef<[u8]>) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
      if let Ok(decoded) = <QCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
        return Ok(RouterCalls::Q(decoded));
      }
      if let Ok(decoded) = <ExecuteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
        return Ok(RouterCalls::Execute(decoded));
      }
      if let Ok(decoded) = <NonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
        return Ok(RouterCalls::Nonce(decoded));
      }
      if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
        return Ok(RouterCalls::Owner(decoded));
      }
      if let Ok(decoded) = <PublicKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
        return Ok(RouterCalls::PublicKey(decoded));
      }
      if let Ok(decoded) = <SetPublicKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
      {
        return Ok(RouterCalls::SetPublicKey(decoded));
      }
      if let Ok(decoded) =
        <UpdatePublicKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
      {
        return Ok(RouterCalls::UpdatePublicKey(decoded));
      }
      if let Ok(decoded) = <VerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
        return Ok(RouterCalls::Verify(decoded));
      }
      Err(ethers::core::abi::Error::InvalidData.into())
    }
  }
  impl ethers::core::abi::AbiEncode for RouterCalls {
    fn encode(self) -> Vec<u8> {
      match self {
        RouterCalls::Q(element) => element.encode(),
        RouterCalls::Execute(element) => element.encode(),
        RouterCalls::Nonce(element) => element.encode(),
        RouterCalls::Owner(element) => element.encode(),
        RouterCalls::PublicKey(element) => element.encode(),
        RouterCalls::SetPublicKey(element) => element.encode(),
        RouterCalls::UpdatePublicKey(element) => element.encode(),
        RouterCalls::Verify(element) => element.encode(),
      }
    }
  }
  impl ::std::fmt::Display for RouterCalls {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
      match self {
        RouterCalls::Q(element) => element.fmt(f),
        RouterCalls::Execute(element) => element.fmt(f),
        RouterCalls::Nonce(element) => element.fmt(f),
        RouterCalls::Owner(element) => element.fmt(f),
        RouterCalls::PublicKey(element) => element.fmt(f),
        RouterCalls::SetPublicKey(element) => element.fmt(f),
        RouterCalls::UpdatePublicKey(element) => element.fmt(f),
        RouterCalls::Verify(element) => element.fmt(f),
      }
    }
  }
  impl ::std::convert::From<QCall> for RouterCalls {
    fn from(var: QCall) -> Self {
      RouterCalls::Q(var)
    }
  }
  impl ::std::convert::From<ExecuteCall> for RouterCalls {
    fn from(var: ExecuteCall) -> Self {
      RouterCalls::Execute(var)
    }
  }
  impl ::std::convert::From<NonceCall> for RouterCalls {
    fn from(var: NonceCall) -> Self {
      RouterCalls::Nonce(var)
    }
  }
  impl ::std::convert::From<OwnerCall> for RouterCalls {
    fn from(var: OwnerCall) -> Self {
      RouterCalls::Owner(var)
    }
  }
  impl ::std::convert::From<PublicKeyCall> for RouterCalls {
    fn from(var: PublicKeyCall) -> Self {
      RouterCalls::PublicKey(var)
    }
  }
  impl ::std::convert::From<SetPublicKeyCall> for RouterCalls {
    fn from(var: SetPublicKeyCall) -> Self {
      RouterCalls::SetPublicKey(var)
    }
  }
  impl ::std::convert::From<UpdatePublicKeyCall> for RouterCalls {
    fn from(var: UpdatePublicKeyCall) -> Self {
      RouterCalls::UpdatePublicKey(var)
    }
  }
  impl ::std::convert::From<VerifyCall> for RouterCalls {
    fn from(var: VerifyCall) -> Self {
      RouterCalls::Verify(var)
    }
  }
  #[doc = "Container type for all return fields from the `Q` function with signature `Q()` and selector `[228, 147, 239, 140]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
    Default,
  )]
  pub struct QReturn(pub ethers::core::types::U256);
  #[doc = "Container type for all return fields from the `execute` function with signature `execute((address,uint256,uint256,bytes)[],(bytes32,bytes32))` and selector `[24, 17, 2, 77]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
    Default,
  )]
  pub struct ExecuteReturn(pub bool);
  #[doc = "Container type for all return fields from the `nonce` function with signature `nonce()` and selector `[175, 254, 208, 224]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
    Default,
  )]
  pub struct NonceReturn(pub ethers::core::types::U256);
  #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
    Default,
  )]
  pub struct OwnerReturn(pub ethers::core::types::Address);
  #[doc = "Container type for all return fields from the `publicKey` function with signature `publicKey()` and selector `[99, 255, 171, 49]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
    Default,
  )]
  pub struct PublicKeyReturn {
    pub parity: u8,
    pub px: [u8; 32],
  }
  #[doc = "Container type for all return fields from the `verify` function with signature `verify(uint8,bytes32,bytes32,bytes32,bytes32)` and selector `[145, 134, 218, 76]`"]
  #[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
    Default,
  )]
  pub struct VerifyReturn(pub bool);
  #[doc = "`RpublicKey(uint8,bytes32)`"]
  #[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
  )]
  pub struct RpublicKey {
    pub parity: u8,
    pub px: [u8; 32],
  }
  #[doc = "`Rsignature(bytes32,bytes32)`"]
  #[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
  )]
  pub struct Rsignature {
    pub e: [u8; 32],
    pub s: [u8; 32],
  }
  #[doc = "`Rtransaction(address,uint256,uint256,bytes)`"]
  #[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
  )]
  pub struct Rtransaction {
    pub to: ethers::core::types::Address,
    pub value: ethers::core::types::U256,
    pub gas: ethers::core::types::U256,
    pub data: ethers::core::types::Bytes,
  }
}
