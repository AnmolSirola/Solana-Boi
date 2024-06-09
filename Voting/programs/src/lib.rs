use anchor_lang::prelude::*;

declare_id!("3Fp6nVU22pfyv3jbLLoDHrj3yaNdKDWoe2qtCtbn38Bf");

#[program]
pub mod voting {
    use super::*;

    pub fn create_poll(ctx: Context<CreatePoll>, poll_question: String) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.question = poll_question;
        poll.creator = *ctx.accounts.creator.key;
        poll.yes_votes = 0;
        poll.no_votes = 0;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, vote: bool) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        let voter = &mut ctx.accounts.voter;

        if voter.voted {
            return Err(ErrorCode::AlreadyVoted.into());
        }

        if vote {
            poll.yes_votes += 1;
        } else {
            poll.no_votes += 1;
        }

        voter.voted = true;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Poll {
    pub question: String,
    pub creator: Pubkey,
    pub yes_votes: u64,
    pub no_votes: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Voter {
    pub voted: bool,
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(init, payer = creator, space = Poll::LEN)]
    pub poll: Account<'info, Poll>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    
    #[account(init_if_needed, payer = voter, space = Voter::LEN)]
    pub voter: Account<'info, Voter>,
    
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You have already voted.")]
    AlreadyVoted,
}

impl Poll {
    pub const LEN: usize = 8 + std::mem::size_of::<Poll>();
}

impl Voter {
    pub const LEN: usize = 8 + std::mem::size_of::<Voter>();
}