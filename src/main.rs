#![cfg_attr(all(not(feature = "export-abi")), no_main, no_std)]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod erc721;
mod int_rk4;
use crate::int_rk4::{tick_many, MotionState};

use crate::erc721::{Erc721, Erc721Params};
use alloc::vec::Vec;
use alloy_primitives::keccak256;
use erc721::IErc721Errors;
use int_rk4::PRECISION;
use stylus_sdk::abi::AbiType;
use stylus_sdk::stylus_proc::{entrypoint, external, sol_storage};
use stylus_sdk::{
    alloy_primitives::U256,
    alloy_sol_types::{sol, SolError, SolType},
    msg,
};
use stylus_sdk::{console, evm};

struct RkFallParams;

impl Erc721Params for RkFallParams {
    const NAME: &'static str = "RkFall NFT";
    const SYMBOL: &'static str = "RKF";
}

sol_storage! {
    #[entrypoint]
    struct RkFall {
        #[borrow]
        Erc721<RkFallParams> erc721;
    }
}

sol! {
    event RkFallMint(uint256 indexed tokenId,uint32 ticks,uint64[] mass,int64[] init_x,int64[] init_y,int64[] init_vel_x,int64[] init_vel_y,int64[] final_x,int64[] final_y,int64[] final_vel_x,int64[] final_vel_y);
    error AlreadyMinted(uint256 tokenId, address owner);
}

pub enum RkFallError {
    AlreadyMinted(AlreadyMinted),
}
impl From<RkFallError> for Vec<u8> {
    fn from(err: RkFallError) -> Vec<u8> {
        match err {
            RkFallError::AlreadyMinted(e) => e.encode(),
        }
    }
}

pub enum CombinedError {
    IErc721Errors(IErc721Errors),
    DynDanceError(RkFallError),
}
impl From<IErc721Errors> for CombinedError {
    fn from(err: IErc721Errors) -> Self {
        CombinedError::IErc721Errors(err)
    }
}

impl From<RkFallError> for CombinedError {
    fn from(err: RkFallError) -> Self {
        CombinedError::DynDanceError(err)
    }
}
impl From<CombinedError> for Vec<u8> {
    fn from(err: CombinedError) -> Vec<u8> {
        match err {
            CombinedError::DynDanceError(e) => RkFallError::into(e),
            CombinedError::IErc721Errors(e) => IErc721Errors::into(e),
        }
    }
}

type CombinedResult<T> = Result<T, CombinedError>;

sol! {
    struct MintParams {
        uint64[] mass;
        int64[] x;
        int64[] y;
        int64[] vel_x;
        int64[] vel_y;
        uint32 ticks;
    }
}

impl AbiType for MintParams {
    type SolType = MintParams;

    // NB: this is the tuple-equivalent of the struct
    const ABI: stylus_sdk::abi::ConstString =
        stylus_sdk::abi::ConstString::new("(uint64[],int64[],int64[],int64[],int64[],uint32)");
}

#[external]
#[inherit(Erc721<RkFallParams>)]
impl RkFall {
    pub fn mint_with_params(&mut self, mint_params: MintParams) -> CombinedResult<U256> {
        self.mint(
            mint_params.mass,
            mint_params.x,
            mint_params.y,
            mint_params.vel_x,
            mint_params.vel_y,
            mint_params.ticks,
        )
    }

    pub fn mint(
        &mut self,
        mass: Vec<u64>,
        x: Vec<i64>,
        y: Vec<i64>,
        vel_x: Vec<i64>,
        vel_y: Vec<i64>,
        ticks: u32,
    ) -> CombinedResult<U256> {
        // initial conditions
        let mut initial_system = Vec::new();
        for i in 0..mass.len() {
            let state = MotionState::new(mass[i], x[i], y[i], vel_x[i], vel_y[i]);
            initial_system.push(state);
        }

        let time_period_sec = (0.001 * PRECISION as f64) as i64;

        // compute the result
        let final_system = tick_many(ticks, time_period_sec, &initial_system);

        // token id is taken from initial conditions
        let data = <sol! { (uint64[], int64[], int64[], int64[], int64[], uint32)}>::encode(&(
            mass.clone(),
            x.clone(),
            y.clone(),
            vel_x.clone(),
            vel_y.clone(),
            ticks,
        ));
        let token_id = keccak256(data).into();
        self.erc721._mint(msg::sender(), token_id)?;

        evm::log(RkFallMint {
            tokenId: token_id,
            ticks: ticks,
            mass: mass,
            init_x: initial_system
                .iter()
                .map(|state| state.get_x())
                .collect::<Vec<i64>>(),
            init_y: initial_system
                .iter()
                .map(|state| state.get_y())
                .collect::<Vec<i64>>(),
            init_vel_x: initial_system
                .iter()
                .map(|state| state.get_vel_x())
                .collect::<Vec<i64>>(),
            init_vel_y: initial_system
                .iter()
                .map(|state| state.get_vel_y())
                .collect::<Vec<i64>>(),
            final_x: final_system
                .iter()
                .map(|state| state.get_x())
                .collect::<Vec<i64>>(),
            final_y: final_system
                .iter()
                .map(|state| state.get_y())
                .collect::<Vec<i64>>(),
            final_vel_x: final_system
                .iter()
                .map(|state| state.get_vel_x())
                .collect::<Vec<i64>>(),
            final_vel_y: final_system
                .iter()
                .map(|state| state.get_vel_y())
                .collect::<Vec<i64>>(),
        });

        Ok(token_id)
    }
}

// #[entrypoint]
// fn user_main(input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
//     Ok(ser)
// }
