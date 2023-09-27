use alloc::{string::String, vec::Vec};

use core::marker::PhantomData;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::{sol, SolError},
    evm, msg,
    stylus_proc::{external, sol_storage},
};

pub trait Erc721Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
}

sol_storage! {
    pub struct Erc721<T> {
        PhantomData<T> const_params;
        mapping(uint256 => address) owners;
        mapping(address => uint256) balances;
        mapping(uint256 => address) token_approvals;
        mapping(address => mapping(address => bool)) operator_approvals;
    }
}

sol! {
    event Transfer(address indexed _from, address indexed _to, uint256 indexed _tokenId);
    event Approval(address indexed _owner, address indexed _approved, uint256 indexed _tokenId);
    event ApprovalForAll(address indexed _owner, address indexed _operator, bool _approved);

    error ERC721NonexistentToken(uint256 tokenId);
    error ERC721IncorrectOwner(address sender, uint256 tokenId, address owner);
    error ERC721InvalidSender(address sender);
    error ERC721InvalidReceiver(address receiver);
    error ERC721InsufficientApproval(address operator, uint256 tokenId);
    error ERC721InvalidApprover(address approver);
    error ERC721InvalidOperator(address operator);
}

pub enum Erc721Error {
    ERC721NonexistentToken(ERC721NonexistentToken),
    ERC721IncorrectOwner(ERC721IncorrectOwner),
    ERC721InvalidSender(ERC721InvalidSender),
    ERC721InvalidReceiver(ERC721InvalidReceiver),
    ERC721InsufficientApproval(ERC721InsufficientApproval),
    ERC721InvalidApprover(ERC721InvalidApprover),
    ERC721InvalidOperator(ERC721InvalidOperator),
}

// CHRIS: TODO: try out the prestwitch way of doing things with .sol
impl From<Erc721Error> for Vec<u8> {
    fn from(err: Erc721Error) -> Vec<u8> {
        match err {
            Erc721Error::ERC721NonexistentToken(e) => e.encode(),
            Erc721Error::ERC721IncorrectOwner(e) => e.encode(),
            Erc721Error::ERC721InvalidSender(e) => e.encode(),
            Erc721Error::ERC721InvalidReceiver(e) => e.encode(),
            Erc721Error::ERC721InsufficientApproval(e) => e.encode(),
            Erc721Error::ERC721InvalidApprover(e) => e.encode(),
            Erc721Error::ERC721InvalidOperator(e) => e.encode(),
        }
    }
}
pub type Erc721Result<T> = Result<T, Erc721Error>;

// CHRIS: TODO: missing errors throughout. Maybe add them now?

impl<T: Erc721Params> Erc721<T> {
    pub fn _approve(
        &mut self,
        to: Address,
        token_id: U256,
        auth: Address,
        emit_event: bool,
    ) -> Erc721Result<()> {
        if emit_event || auth != Address::ZERO {
            let owner = self.owner_of(token_id)?;

            if auth != Address::ZERO && owner != auth && !self.is_approved_for_all(owner, auth)? {
                return Err(Erc721Error::ERC721InvalidApprover(ERC721InvalidApprover {
                    approver: auth,
                }));
            }

            if emit_event {
                evm::log(Approval {
                    _owner: owner,
                    _approved: auth,
                    _tokenId: token_id,
                })
            }
        }

        self.token_approvals.setter(token_id).set(to);

        Ok(())
    }

    pub fn _require_minted(&self, token_id: U256) -> Erc721Result<()> {
        if self.owner_of(token_id)? == Address::ZERO {
            return Err(Erc721Error::ERC721NonexistentToken(
                ERC721NonexistentToken { tokenId: token_id },
            ));
        }

        Ok(())
    }

    pub fn _is_authorized(
        &self,
        owner: Address,
        spender: Address,
        token_id: U256,
    ) -> Erc721Result<bool> {
        Ok(spender != Address::ZERO
            && (owner == spender
                || self.is_approved_for_all(owner, spender)?
                || self.get_approved(token_id)? == spender))
    }

    pub fn _check_authorized(
        &self,
        owner: Address,
        spender: Address,
        token_id: U256,
    ) -> Erc721Result<()> {
        if !self._is_authorized(owner, spender, token_id)? {
            if owner == Address::ZERO {
                return Err(Erc721Error::ERC721NonexistentToken(
                    ERC721NonexistentToken { tokenId: token_id },
                ));
            } else {
                return Err(Erc721Error::ERC721InsufficientApproval(
                    ERC721InsufficientApproval {
                        operator: spender,
                        tokenId: token_id,
                    },
                ));
            }
        }

        Ok(())
    }

    pub fn _update(&mut self, to: Address, token_id: U256, auth: Address) -> Erc721Result<Address> {
        let from = self.owner_of(token_id)?;

        if auth != Address::ZERO {
            self._check_authorized(from, auth, token_id)?;
        }

        if from != Address::ZERO {
            self._approve(Address::ZERO, token_id, Address::ZERO, false)?;

            // CHRIS: TODO: check what happens for overflows
            let new_bal = self.balances.get(from) - U256::from(1);
            self.balances.setter(from).set(new_bal);
        }

        if to != Address::ZERO {
            let new_bal = self.balances.get(to) + U256::from(1);
            self.balances.setter(to).set(new_bal);
        }

        self.owners.setter(token_id).set(to);

        evm::log(Transfer {
            _from: from,
            _to: to,
            _tokenId: token_id,
        });

        Ok(from)
    }

    pub fn _mint(&mut self, to: Address, token_id: U256) -> Erc721Result<()> {
        if to == Address::ZERO {
            return Err(Erc721Error::ERC721InvalidReceiver(ERC721InvalidReceiver {
                receiver: Address::ZERO,
            }));
        }

        let prev_owner = self._update(to, token_id, Address::ZERO)?;
        if prev_owner != Address::ZERO {
            return Err(Erc721Error::ERC721InvalidSender(ERC721InvalidSender {
                sender: Address::ZERO,
            }));
        }

        Ok(())
    }

    pub fn _burn(&mut self, token_id: U256) -> Erc721Result<()> {
        let prev_owner = self._update(Address::ZERO, token_id, Address::ZERO)?;
        if prev_owner == Address::ZERO {
            return Err(Erc721Error::ERC721NonexistentToken(
                ERC721NonexistentToken { tokenId: token_id },
            ));
        }
        Ok(())
    }
}

#[external]
impl<T: Erc721Params> Erc721<T> {
    pub fn name() -> Erc721Result<String> {
        Ok(T::NAME.into())
    }

    pub fn symbol() -> Erc721Result<String> {
        Ok(T::SYMBOL.into())
    }

    pub fn balance_of(&self, owner: Address) -> Erc721Result<U256> {
        Ok(self.balances.get(owner))
    }

    pub fn owner_of(&self, token_id: U256) -> Erc721Result<Address> {
        // CHRIS: TODO: should throw if no owner found rather tha returning address(0)
        Ok(self.owners.get(token_id))
    }

    // CHRIS: TODO: fill this later
    // pub fn supports_interface
    // pub fn token_uri()
    // pub fn base_uri()
    // pub fn safe_transfer_from(
    //     &mut self, from: Address, to: Address, token_id: U256, data: Bytes
    // )

    pub fn approve(&mut self, to: Address, token_id: U256) -> Erc721Result<()> {
        self._approve(to, token_id, msg::sender(), true)?;

        Ok(())
    }

    pub fn is_approved_for_all(&self, owner: Address, operator: Address) -> Erc721Result<bool> {
        Ok(self.operator_approvals.get(owner).get(operator))
    }

    pub fn get_approved(&self, token_id: U256) -> Erc721Result<Address> {
        self._require_minted(token_id)?;

        Ok(self.token_approvals.get(token_id))
    }

    pub fn set_approval_for_all(&mut self, operator: Address, approved: bool) -> Erc721Result<()> {
        if operator == Address::ZERO {
            return Err(Erc721Error::ERC721InvalidOperator(ERC721InvalidOperator {
                operator: Address::ZERO,
            }));
        }

        let owner = msg::sender();
        self.operator_approvals
            .setter(owner)
            .setter(operator)
            .set(approved);

        evm::log(ApprovalForAll {
            _owner: owner,
            _operator: operator,
            _approved: approved,
        });

        Ok(())
    }

    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        token_id: U256,
    ) -> Erc721Result<()> {
        if to == Address::ZERO {
            return Err(Erc721Error::ERC721InvalidReceiver(ERC721InvalidReceiver {
                receiver: Address::ZERO,
            }));
        }

        let prev_owner = self._update(to, token_id, msg::sender())?;

        if prev_owner != from {
            return Err(Erc721Error::ERC721IncorrectOwner(ERC721IncorrectOwner {
                sender: from,
                tokenId: token_id,
                owner: prev_owner,
            }));
        }

        Ok(())
    }

    pub fn safe_transfer_from(
        &mut self,
        from: Address,
        to: Address,
        token_id: U256,
    ) -> Erc721Result<()> {
        Ok(self.transfer_from(from, to, token_id)?)
    }
}
