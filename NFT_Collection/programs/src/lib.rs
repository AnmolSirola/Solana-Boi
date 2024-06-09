use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

declare_id!("8difjqmp8VJs5dSVExiWNgQC45ZKmmASTnUfqCLrBfVN");

#[program]
pub mod nft_collection {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>, metadata_uri: String) -> Result<()> {
        // Mint the NFT token
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        // Create metadata account
        let mut creators: Vec<mpl_token_metadata::state::Creator> =
            vec![mpl_token_metadata::state::Creator {
                address: ctx.accounts.authority.key(),
                verified: true,
                share: 100,
            }];

        let metadata_accounts = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let data = mpl_token_metadata::state::DataV2 {
            name: "NFT Name".to_string(),
            symbol: "NFT".to_string(),
            uri: metadata_uri,
            seller_fee_basis_points: 0,
            creators: Some(creators.clone()),
            collection: None,
            uses: None,
        };

        let create_metadata_accounts_infos = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        mpl_token_metadata::state::TokenMetadataAccount::create_v3(
            ctx.accounts.token_metadata_program.key,
            ctx.accounts.metadata.key,
            ctx.accounts.mint.key,
            ctx.accounts.mint_authority.key,
            ctx.accounts.authority.key,
            ctx.accounts.authority.key,
            data.name,
            data.symbol,
            data.uri,
            Some(creators),
            0,
            true,
            false,
            None,
            None,
        );

        // Create master edition account
        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        create_master_edition_v3(
            master_edition_infos.as_slice(),
            CreateMasterEditionV3 {
                max_supply: Some(0),
            },
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(init, payer = authority, mint::decimals = 0, mint::authority = authority)]
    pub mint: Account<'info, Mint>,

    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

    #[account(mut)]
    pub metadata: AccountInfo<'info>,

    #[account(mut)]
    pub master_edition: AccountInfo<'info>,

    pub mint_authority: AccountInfo<'info>,
    pub token_metadata_program: AccountInfo<'info>,
}