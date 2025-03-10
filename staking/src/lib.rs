#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U256, U8, U32, U64 };
use stylus_sdk::{ prelude::*, msg, evm, block };
use alloy_sol_types::sol;
// so the nft will be minted with the gallery it was in
// and the index of its resulting crowd staking

// Define the leaderboard size; sets the top casters in the ordeal
const LEADERBOARD_SIZE: u8 = 30;

sol_storage! {
    #[entrypoint]
    // holds the staking record to the gallery indentity
    pub struct Stake{
        // gallery index to the room; <room to gallery index>
        mapping(uint256 =>  Gallery) room;

        // here will hold if user has voted
        mapping(address => mapping(uint256 => bool)) voted;
        // this will be used for controled staking
        address stake_control;
        address admin; // holds the address of the admin
        // nft to result of stake
    }

// this is the gallery room property
// holding data of stakes for each nft under a gallery
    pub struct Gallery {
        // total vote in the gallery
        uint256 total_votes;

        // this will map the nft id to the votes of that nft<from the nft_libary accepted nfts>
        mapping(uint256 => Nft) nft;
        
    }

    pub struct Nft {
        mapping(uint8 => uint256) leaderboard; // this is the ranking stakers for the nft
        uint256 total_votes; // this is the total vots of the nft
        // index casted votes
        mapping(uint256 => Cast ) casted;  // this is the identy of each of the user that has casted a vot
        // therefore each vote has an idnetity 

    }

    // this is the vote proterty< therefore each vote holds an identity and property>
    pub struct Cast {
        uint256 bid; // this is the current value of their vote
        uint32 updated; //this is the time the cater updated the value of his vote
        address voter; //this is the address of the voter
    }


}

sol! {
    // event to show that a new gallary have been created
    event Stakes(address indexed voter, uint256 indexed gallery_id, uint256 indexed  nft_id, uint256 bid, uint64 time);
    event UpdatedCast(address indexed voter, uint256 indexed gallery_id, uint256 indexed nft_id, uint256 old_bid, uint256 new_bid, uint64 time);
    error InvalidParameter(uint8 point);
}

#[derive(SolidityError)]
pub enum StakeError {
    InvalidParameter(InvalidParameter),
}

#[public]
impl Stake {
    // this is the unsafe stake mechanism
    // this will only be allowed to be called by a safe contract <safe_vote>
    pub fn stake(
        &mut self,
        user: Address, // address of the individual that is casting the vote
        gallery_id: U256, // gallery index of which the nft belongs to
        nft_id: U256, // identity of the nft in the gallery
        bid: U256 //vote value
    ) -> Result<(), StakeError> {
        // this makes sure that ony the safew contract can call this function
        if msg::sender() != self.stake_control.get() {
            return Err(
                StakeError::InvalidParameter(InvalidParameter {
                    point: 3,
                })
            );
        }

        // checks to make sure that the user has not voted before
        if self.has_voted(gallery_id, user) {
            return Err(
                StakeError::InvalidParameter(InvalidParameter {
                    point: 4,
                })
            );
        }

        let mut gallery = self.room.setter(gallery_id); //gets the gallery identity
        let mut nft = gallery.nft.setter(nft_id); //gets the nft form the gallery

        // Use the total_votes to determine the index for the new vote
        // starting the index from 1 and not 0, so that it will not be affected  none value index
        let available_index = nft.total_votes.get() + U256::from(1); // increases the total votes // the total votes also serves as a point of available index for a cast

        // Add the new vote to the casted mapping
        {
            let mut cast_vote = nft.casted.setter(available_index); // setting a new cast identity
            cast_vote.bid.set(bid);
            cast_vote.updated.set(U32::from(block::timestamp()));
            cast_vote.voter.set(user);
        }

        // Collect data for the leaderboard update and release the mutable borrow
        // increases the total votes
        nft.total_votes.set(available_index); // sets new votes

        let gallery_total_vote = gallery.total_votes.get(); // increase the totalvotes in the gallery
        gallery.total_votes.set(gallery_total_vote + U256::from(1));

        // Call update_le_nft after releasing the mutable borrow of `nft`
        self.update_le_nft(gallery_id, nft_id, available_index, bid); // updates the leaderboard
        self.voted.setter(user).setter(gallery_id).set(true); // sets a new vote state for the user; making sure that user can not vote again in this given gallery
        // but can only increase their cast

        // inform that the cast has happened
        evm::log(Stakes {
            voter: user,
            gallery_id,
            nft_id,
            bid,
            time: block::timestamp() as u64,
        });
        Ok(())
    }

    // unsafebid update
    // where users can update the value of their bid
    pub fn update_bid(
        &mut self,
        user: Address, // address of the user, whoese bid is to be upgraded
        gallery_id: U256, // gallery index of the nft
        nft_id: U256, // identity of the nft
        vote_id: U256, // identity of the casted vote
        bid: U256 // new bid
    ) -> Result<(), StakeError> {
        // makes sure that only the safe contract can call this function
        if msg::sender() != self.stake_control.get() {
            return Err(
                StakeError::InvalidParameter(InvalidParameter {
                    point: 11,
                })
            );
        }

        let mut gallery = self.room.setter(gallery_id); // gets the gallery
        let mut nft = gallery.nft.setter(nft_id); //gets the nft
        let mut cast_vote = nft.casted.setter(vote_id); //get the casted vote

        // make sure that the user being passed is the same user that owns the cast identity
        if user != cast_vote.voter.get() {
            return Err(
                StakeError::InvalidParameter(InvalidParameter {
                    point: 15,
                })
            );
        }
        // gets the old bid for the event
        let old_bid = cast_vote.bid.get();
        // setting the new bid
        cast_vote.bid.set(bid);
        cast_vote.updated.set(U32::from(block::timestamp())); //set time of operation
        self.update_le_nft(gallery_id, nft_id, vote_id, bid); // attempt to update the leaderboard

        // emit the operation
        evm::log(UpdatedCast {
            voter: user,
            gallery_id,
            nft_id,
            old_bid,
            new_bid: bid,
            time: block::timestamp() as u64,
        });
        Ok(())
    }

    //get total vote of a particular nft
    pub fn get_total_votes(&self, gallery_id: U256, nft_id: U256) -> U256 {
        let gallery = self.room.getter(gallery_id);
        let nft = gallery.nft.getter(nft_id);
        nft.total_votes.get()
    }

    // get the information of a cast, using the vote id
    pub fn get_cast(&self, gallery_id: U256, nft_id: U256, vote_id: U256) -> (U256, u32, Address) {
        let gallery = self.room.getter(gallery_id);
        let nft = gallery.nft.getter(nft_id);
        let cast_vote = nft.casted.getter(vote_id);
        // out put (vote value, last time the vote was updated, owner of the cast or who casted the vote)
        (cast_vote.bid.get(), cast_vote.updated.get().to::<u32>(), cast_vote.voter.get())
    }

    // this gives a list of the leadrboard within a range;
    //  the list is arranged form the hightest bid to the lowest bid
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
                let vote_id = nft.leaderboard.getter(U8::from(i as u8)).get(); // getting the vote id
                let bid = nft.casted.getter(vote_id).bid.get(); // the bid amount
                let voter = nft.casted.getter(vote_id).voter.get(); //the voter
                let updated = nft.casted.getter(vote_id).updated.get().to::<u32>(); //last time the bid was updated
                //output (vote id, vote value, voter, last time the vote was updated)
                (vote_id, bid, voter, updated)
            })
            .collect();
        leaderboard
    }

    // this is to set the admin and control the contract to make sure that only the admin can set a safe contract
    pub fn set_control(&mut self, stake: Address) -> Result<(), StakeError> {
        self.check_admin().map_err(|e| { e })?; //makes sure only the admin can call this function
        self.stake_control.set(stake);
        Ok(())
    }

    // this is to check if the user has voted
    pub fn has_voted(&self, gallery_id: U256, user: Address) -> bool {
        let ux = self.voted.getter(user);
        let gx = ux.getter(gallery_id);
        gx.get()
    }

    // this is to get the total votes in the gallery
    pub fn get_gallery_total_votes(&self, gallery_id: U256) -> U256 {
        self.room.getter(gallery_id).total_votes.get()
    }

    pub fn get_position(
        &self,
        gallery_id: U256,
        nft_id: U256,
        user: Address
    ) -> Result<u8, StakeError> {
        let room = self.room.getter(gallery_id);
        let nft = room.nft.getter(nft_id);
        // Assuming the leaderboard has a defined size
        for i in 0..=LEADERBOARD_SIZE {
            let cast = nft.leaderboard.getter(U8::from(i)).get();
            let casted = nft.casted.getter(cast);
            let board_voter = casted.voter.get();
            // if casted.bid.get() == U256::from(0){
            //     return  // If the user is not found
            //     Err(
            //         StakeError::InvalidParameter(InvalidParameter {
            //             point: 7, // Consider making this a more meaningful error code or message
            //         })
            //     )
            // }
            if board_voter == user {
                return Ok(i as u8);
            }
        }

        return Ok(LEADERBOARD_SIZE + 3);
    }
}

impl Stake {
    // this function is responsible for the arrangement of the leaderboard
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

        // Check if the vote_id already exists
        if let Some(entry) = leaderboard.iter_mut().find(|entry| entry.0 == vote_id) {
            // Update the existing bid if the new bid is higher
            entry.1 = entry.1.max(bid);
        } else {
            // Add the new vote if it doesn't exist
            leaderboard.push((vote_id, bid));
        }

        // Sort the leaderboard by bid in descending order
        leaderboard.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by bid in descending order
        leaderboard.truncate(LEADERBOARD_SIZE.into()); // Keep only the top N entries

        // Update the leaderboard in storage
        for (i, (vote_id, _)) in leaderboard.iter().enumerate() {
            nft.leaderboard.setter(U8::from(i as u8)).set(*vote_id);
        }
    }

    // this is the check to make sure that the admin is calling a particular function and no one else
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
