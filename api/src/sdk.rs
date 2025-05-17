use steel::*;
use crankx::Solution;

use crate::{
    consts::*, 
    instruction::*, 
    pda::*,
    utils,
};

pub fn build_create_ix(
    signer: Pubkey,
    name: &str,
) -> Instruction {
    let name = utils::to_name(name);
    let (tape_address, _tape_bump) = tape_pda(signer, &name);
    let (writer_address, _writer_bump) = writer_pda(tape_address);

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(tape_address, false),
            AccountMeta::new(writer_address, false),
            AccountMeta::new_readonly(solana_program::system_program::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(sysvar::slot_hashes::ID, false),
        ],
        data: Create {
            name,
        }.to_bytes(),
    }
}

pub fn build_write_ix(
    signer: Pubkey,
    tape: Pubkey,
    writer: Pubkey,
    prev_segment: Option<[u8; 64]>,
    data: &[u8],
) -> Instruction {

    let prev_segment = match prev_segment {
        Some(sig) => sig,
        None => [0; 64],
    };

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(tape, false),
            AccountMeta::new(writer, false),
        ],
        data: Write::new(
            prev_segment
        ).pack(data),
    }
}

pub fn build_finalize_ix(
    signer: Pubkey, 
    tape: Pubkey,
    writer: Pubkey,
    tail_segment: [u8; 64],
) -> Instruction {

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(tape, false),
            AccountMeta::new(writer, false),
            AccountMeta::new(ARCHIVE_ADDRESS, false),
            AccountMeta::new_readonly(solana_program::system_program::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data: Finalize {
            tail: tail_segment,
        }.to_bytes(),
    }
}

pub fn build_register_ix(
    signer: Pubkey, 
    name: &str
) -> Instruction {
    let name = utils::to_name(name);
    let (miner_address, _bump) = miner_pda(signer, name);

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(miner_address, false),
            AccountMeta::new_readonly(ARCHIVE_ADDRESS, false),
            AccountMeta::new_readonly(solana_program::system_program::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
            AccountMeta::new_readonly(sysvar::slot_hashes::ID, false),
        ],
        data: Register {
            name,
        }.to_bytes(),
    }
}

pub fn build_mine_ix(
    signer: Pubkey,
    miner: Pubkey,
    spool: Pubkey,
    tape: Pubkey,
    solution: Solution,
    recall_chunk: [u8; CHUNK_SIZE],
    recall_proof: [[u8;32]; PROOF_LEN],
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(spool, false),
            AccountMeta::new(miner, false),
            AccountMeta::new_readonly(tape, false),
            AccountMeta::new_readonly(EPOCH_ADDRESS, false),
            AccountMeta::new_readonly(ARCHIVE_ADDRESS, false),
            AccountMeta::new_readonly(sysvar::slot_hashes::ID, false),
        ],
        data: Mine {
            digest: solution.d,
            nonce: solution.n,
            recall_chunk,
            recall_proof,
        }.to_bytes(),
    }
}

pub fn build_claim_ix(
    signer: Pubkey, 
    miner: Pubkey,
    beneficiary: Pubkey, 
    amount: u64
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(beneficiary, false),
            AccountMeta::new(miner, false),
            AccountMeta::new_readonly(TREASURY_ADDRESS, false),
            AccountMeta::new(TREASURY_ATA, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: Claim {
            amount: amount.to_le_bytes(),
        }.to_bytes(),
    }
}

pub fn build_close_ix(
    signer: Pubkey,
    miner: Pubkey,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(miner, false),
            AccountMeta::new_readonly(solana_program::system_program::ID, false),
        ],
        data: Close {}.to_bytes(),
    }
}

pub fn build_initialize_ix(
    signer: Pubkey
) -> Instruction {
    let spool_pdas = [
        spool_pda(0).0,
        spool_pda(1).0,
        spool_pda(2).0,
        spool_pda(3).0,
        spool_pda(4).0,
        spool_pda(5).0,
        spool_pda(6).0,
        spool_pda(7).0,
    ];

    let (archive_pda, _archive_bump) = archive_pda();
    let (epoch_pda, _epoch_bump) = epoch_pda();
    let (mint_pda, _mint_bump) = mint_pda();
    let (treasury_pda, _treasury_bump) = treasury_pda();
    let (treasury_ata, _treasury_ata_bump) = treasury_ata();
    let (metadata_pda, _metadata_bump) = metadata_pda(mint_pda);

    assert_eq!(archive_pda, ARCHIVE_ADDRESS);
    assert_eq!(epoch_pda, EPOCH_ADDRESS);
    assert_eq!(mint_pda, MINT_ADDRESS);
    assert_eq!(treasury_pda, TREASURY_ADDRESS);
    assert_eq!(treasury_ata, TREASURY_ATA);

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(spool_pdas[0], false),
            AccountMeta::new(spool_pdas[1], false),
            AccountMeta::new(spool_pdas[2], false),
            AccountMeta::new(spool_pdas[3], false),
            AccountMeta::new(spool_pdas[4], false),
            AccountMeta::new(spool_pdas[5], false),
            AccountMeta::new(spool_pdas[6], false),
            AccountMeta::new(spool_pdas[7], false),
            AccountMeta::new(archive_pda, false),
            AccountMeta::new(epoch_pda, false),
            AccountMeta::new(metadata_pda, false),
            AccountMeta::new(mint_pda, false),
            AccountMeta::new(treasury_pda, false),
            AccountMeta::new(treasury_ata, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data: Initialize {}.to_bytes(),
    }
}

pub fn build_advance_ix(
    signer: Pubkey
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(SPOOL_ADDRESSES[0], false),
            AccountMeta::new(SPOOL_ADDRESSES[1], false),
            AccountMeta::new(SPOOL_ADDRESSES[2], false),
            AccountMeta::new(SPOOL_ADDRESSES[3], false),
            AccountMeta::new(SPOOL_ADDRESSES[4], false),
            AccountMeta::new(SPOOL_ADDRESSES[5], false),
            AccountMeta::new(SPOOL_ADDRESSES[6], false),
            AccountMeta::new(SPOOL_ADDRESSES[7], false),
            AccountMeta::new(EPOCH_ADDRESS, false),
            AccountMeta::new(MINT_ADDRESS, false),
            AccountMeta::new(TREASURY_ADDRESS, false),
            AccountMeta::new(TREASURY_ATA, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: Advance {}.to_bytes(),
    }
}
