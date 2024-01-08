use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;


declare_id!("CEtJEtnhHw4VJQUbEAd4u2fUBjxiAUpnszmR6TNjsxWE");

#[program]
pub mod mycalculator_dapp {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }

    pub fn create(ctx: Context<Create>, init_message: String ) ->  ProgramResult{
        let calculator  = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn add(ctx:Context<Addition>, num1: i64, num2: i64)-> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result=num1+num2;
        Ok(())
    }

    pub fn subtract(ctx:Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result=num1-num2;
        Ok(())
    }

    pub fn multiply(ctx:Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result=num1*num2;
        Ok(())
    }

    pub fn divide(ctx:Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result=num1/num2;
        calculator.remainder = num1%num2;
        Ok(())
    }
}




#[derive(Accounts)]
pub struct Create<'info>{
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}


// Add two numbers
#[derive(Accounts)]
pub struct Addition<'info>{
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

// Subtract two numbers
#[derive(Accounts)]
pub struct Subtraction<'info>{
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

// Multiply two numbers
#[derive(Accounts)]
pub struct Multiplication<'info>{
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

// Divide two numbers
#[derive(Accounts)]
pub struct Division<'info>{
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator{
    pub greeting: String,
    pub result: i64,
    pub remainder: i64

}
