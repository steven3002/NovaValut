#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U256, U8 };
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
        // Get mutable references to the gallery and NFT
        let mut gallery = self.room.setter(gallery_id);
        let mut nft = gallery.nft.setter(nft_id);

        // Cache the top 4 leaderboard positions
        let mut leaderboard = [
            nft.leaderboard.getter(U8::from(0)).get(),
            nft.leaderboard.getter(U8::from(1)).get(),
            nft.leaderboard.getter(U8::from(2)).get(),
            nft.leaderboard.getter(U8::from(3)).get(),
        ];

        // Cache the bids for the top 4 positions
        let mut bids = [
            nft.casted.getter(leaderboard[0]).bid.get(),
            nft.casted.getter(leaderboard[1]).bid.get(),
            nft.casted.getter(leaderboard[2]).bid.get(),
            nft.casted.getter(leaderboard[3]).bid.get(),
        ];

        // Check if the new bid qualifies for the leaderboard
        for i in 0..4 {
            if bid > bids[i] {
                // Shift lower bids down the leaderboard
                for j in (i + 1..4).rev() {
                    leaderboard[j] = leaderboard[j - 1];
                    bids[j] = bids[j - 1];
                }
                // Insert the new vote
                leaderboard[i] = vote_id;
                bids[i] = bid;
                break;
            }
        }

        // Update the leaderboard in storage
        for (i, vote_id) in leaderboard.iter().enumerate() {
            nft.leaderboard.setter(U8::from(i as u8)).set(*vote_id);
        }
    }
}
