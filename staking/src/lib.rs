#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U256, U8, U32 };
use stylus_sdk::{ prelude::*, msg, evm, block };
use alloy_sol_types::sol;

// so the nft will be minted with the gallery it was in
// and the index of its resulting crowd staking

// Define the leaderboard size
const LEADERBOARD_SIZE: u8 = 30;

sol_storage! {
    #[entrypoint]
    pub struct Stake{
        // gallery index to 
        mapping(uint256 =>  Gallery) room;

        // here will hold if user has voted
        mapping(address => mapping(uint256 => bool)) voted;
        // this will be used for controled staking
        address stake_control;
        address admin;
        // nft to result of stake
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

sol! {
    // event to show that a new gallary have been created
    event Stakes(address indexed voter, uint256 indexed gallery_id, uint256 nft_id, uint256 bid);
    event UpdatedCast(address indexed voter, uint256 indexed gallery_id, uint256 nft_id, uint256 old_bid, uint256 new_bid);
    error InvalidParameter(uint8 point);
}

#[derive(SolidityError)]
pub enum StakeError {
    InvalidParameter(InvalidParameter),
}

#[public]
impl Stake {
    pub fn stake(
        &mut self,
        user: Address,
        gallery_id: U256,
        nft_id: U256,
        bid: U256
    ) -> Result<(), StakeError> {
        // Get mutable references to the gallery and NFT
        if msg::sender() != self.admin.get() {
            return Err(
                StakeError::InvalidParameter(InvalidParameter {
                    point: 3,
                })
            );
        }

        if self.has_voted(gallery_id, user) {
            return Err(
                StakeError::InvalidParameter(InvalidParameter {
                    point: 4,
                })
            );
        }

        let mut gallery = self.room.setter(gallery_id);
        let mut nft = gallery.nft.setter(nft_id);

        // Use the total_votes to determine the index for the new vote
        let available_index = nft.total_votes.get();

        // Add the new vote to the casted mapping
        {
            let mut cast_vote = nft.casted.setter(available_index);
            cast_vote.bid.set(bid);
            cast_vote.updated.set(U32::from(block::timestamp()));
            cast_vote.voter.set(user);
        }

        // Collect data for the leaderboard update and release the mutable borrow
        let total_votes = nft.total_votes.get() + U256::from(1);
        nft.total_votes.set(total_votes);

        let gallery_total_vote = gallery.total_votes.get();
        gallery.total_votes.set(gallery_total_vote + U256::from(1));

        // Call update_le_nft after releasing the mutable borrow of `nft`
        self.update_le_nft(gallery_id, nft_id, available_index, bid);
        self.voted.setter(user).setter(gallery_id).set(true);

        evm::log(Stakes {
            voter: user,
            gallery_id,
            nft_id,
            bid,
        });
        Ok(())
    }

    pub fn update_bid(
        &mut self,
        user: Address,
        gallery_id: U256,
        nft_id: U256,
        vote_id: U256,
        bid: U256
    ) -> Result<(), StakeError> {
        if msg::sender() != self.admin.get() {
            return Err(
                StakeError::InvalidParameter(InvalidParameter {
                    point: 11,
                })
            );
        }

        let mut gallery = self.room.setter(gallery_id);
        let mut nft = gallery.nft.setter(nft_id);
        let mut cast_vote = nft.casted.setter(vote_id);
        if user != cast_vote.voter.get() {
            return Err(
                StakeError::InvalidParameter(InvalidParameter {
                    point: 15,
                })
            );
        }
        let old_bid = cast_vote.bid.get();
        cast_vote.bid.set(bid);
        cast_vote.updated.set(U32::from(block::timestamp()));
        self.update_le_nft(gallery_id, nft_id, vote_id, bid);
        evm::log(UpdatedCast {
            voter: user,
            gallery_id,
            nft_id,
            old_bid,
            new_bid: bid,
        });
        Ok(())
    }

    pub fn get_total_votes(&self, gallery_id: U256, nft_id: U256) -> U256 {
        let gallery = self.room.getter(gallery_id);
        let nft = gallery.nft.getter(nft_id);
        nft.total_votes.get()
    }

    pub fn get_cast(&self, gallery_id: U256, nft_id: U256, vote_id: U256) -> (U256, u32, Address) {
        let gallery = self.room.getter(gallery_id);
        let nft = gallery.nft.getter(nft_id);
        let cast_vote = nft.casted.getter(vote_id);
        (cast_vote.bid.get(), cast_vote.updated.get().to::<u32>(), cast_vote.voter.get())
    }
    // these are the variables to be returned
    // (vote_id, bid, address)
    pub fn get_leaderboard(
        &self,
        gallery_id: U256,
        nft_id: U256,
        start: u8,
        end: u8
    ) -> Vec<(U256, U256, Address, u32)> {
        let gallery = self.room.getter(gallery_id);
        let nft = gallery.nft.getter(nft_id);

        // Fetch current leaderboard
        let leaderboard: Vec<(U256, U256, Address, u32)> = (start..end)
            .map(|i| {
                let vote_id = nft.leaderboard.getter(U8::from(i as u8)).get();
                let bid = nft.casted.getter(vote_id).bid.get();
                let voter = nft.casted.getter(vote_id).voter.get();
                let updated = nft.casted.getter(vote_id).updated.get().to::<u32>();
                (vote_id, bid, voter, updated)
            })
            .collect();
        leaderboard
    }

    pub fn set_control(&mut self, stake: Address) -> Result<(), StakeError> {
        self.check_admin().map_err(|e| { e })?;
        self.stake_control.set(stake);
        Ok(())
    }

    pub fn has_voted(&self, gallery_id: U256, user: Address) -> bool {
        let ux = self.voted.getter(user);
        let gx = ux.getter(gallery_id);
        gx.get()
    }

    pub fn get_gallery_total_votes(&self, gallery_id: U256) -> U256 {
        self.room.getter(gallery_id).total_votes.get()
    }
}

impl Stake {
    pub fn update_le_nft(&mut self, gallery_id: U256, nft_id: U256, vote_id: U256, bid: U256) {
        // Get mutable references to the gallery and NFT
        let mut gallery = self.room.setter(gallery_id);
        let mut nft = gallery.nft.setter(nft_id);

        // Fetch current leaderboard
        let mut leaderboard: Vec<(U256, U256)> = (0..LEADERBOARD_SIZE)
            .map(|i| {
                let vote_id = nft.leaderboard.getter(U8::from(i as u8)).get();
                let bid = nft.casted.getter(vote_id).bid.get();
                (vote_id, bid)
            })
            .collect();

        // Insert the new vote if it qualifies
        leaderboard.push((vote_id, bid));
        leaderboard.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by bid in descending order
        leaderboard.truncate(LEADERBOARD_SIZE.into()); // Keep only the top N entries

        // Update the leaderboard in storage
        for (i, (vote_id, _)) in leaderboard.iter().enumerate() {
            nft.leaderboard.setter(U8::from(i as u8)).set(*vote_id);
        }
    }

    pub fn check_admin(&mut self) -> Result<bool, StakeError> {
        let default_x = Address::from([0x00; 20]);
        if self.admin.get() != default_x && msg::sender() != self.admin.get() {
            return Err(
                StakeError::InvalidParameter(InvalidParameter {
                    point: 0,
                })
            );
        } else if self.admin.get() == default_x {
            self.admin.set(msg::sender());
            return Ok(true);
        } else {
            return Ok(true);
        }
    }
}
