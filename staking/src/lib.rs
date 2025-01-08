#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U256 };
use stylus_sdk::{ prelude::*, msg, evm };
use alloy_sol_types::sol;

use stylus_sdk::call::Call;

// so the nft will be minted with the gallery it was in
// and the index of its resulting crowd staking

sol_storage! {
    #[entrypoint]
    pub struct Stake{
        // gallery index to 
        mapping(uint256 =>  Gallery) room;
        // nft to result of stake
        // mapping(uint256 )
    }


    pub struct Gallery {
        uint256 total_votes;

        // this will map the nft id to the votes of that nft
        mapping(uint256 => Nft) nft;
        
    }

    pub struct Nft {
        mapping(uint8 => uint256) leaderboard;
        uint256 total_votes;
        // index casted votes
        mapping(uint256 => Cast ) casted;

    }

    pub struct Cast {
        uint256 bid;
        uint32 updated;
        address voter;
    }
    // this will check the nft libary to check if a particular nft exist

}

#[public]
impl Stake {
    // so todo
    // so we will check if the nft exist and if the gallery exist;
    // then we will check if they have voted; then we will check if they want to increase their bid or; add new bid
    // check the erc contract to confirm that the transaction
    //  we will update the storage

    // view functions

    // for the view functions
    // one will be able to see the total vots for the nfts and can call to see the top 5 votes

    // pub

}

impl Stake {
    pub fn update_le_nft(&mut self, gallery_id: U256, nft_id: U256, vote_id: U256, bid: U256) {
        // here we will only for with the top 4

        let mut gallery = self.room.setter(gallery_id);
        let mut nft = gallery.nft.setter(nft_id);
        // first position on the leaderboard
        let mut first = nft.leaderboard.setter(U8::from(0));
        let mut secound = nft.leaderboard.setter(U8::from(1));
        let mut third = nft.leaderboard.setter(U8::from(2));
        let mut fourth = nft.leaderboard.setter(U8::from(3));

        if bid > nft.casted.getter(first.get()).bid.get() {
            // we will update the leaderboard
            first.set(vote_id);
            return;
        } else if bid > nft.casted.getter(secound.get()).bid.get() {
            secound.set(vote_id);
            return;
        } else if bid > nft.casted.getter(third.get()).bid.get() {
            third.set(vote_id);
            return;
        } else if bid > nft.casted.getter(fourth.get()).bid.get() {
            fourth.set(vote_id);
            return;
        }
        return;
    }
}
