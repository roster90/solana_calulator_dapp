const assert = require('assert');
const anchor = require('@project-serum/anchor');
const { describe } = require('mocha');
const {SystemProgram} = anchor.web3;


describe('calculator_dapp', ()=>{
  const provider = anchor.AnchorProvider.local();


  anchor.setProvider(provider);
  const calculator = anchor.web3.Keypair.generate();
  const program = anchor.workspace.MycalculatorDapp;
  it('Create a calculator', async()=>{
    await program.rpc.create('Welcome to solana',{
      accounts:{
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [calculator],

    } );
    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.greeting == "Welcome to solana");
  })

  it('Adds 2 numbers', async()=>{
   await program.rpc.add(new anchor.BN(2), new anchor.BN(3),{
    accounts:{
      calculator: calculator.publicKey
    }
   })
   const account = await program.account.calculator.fetch(calculator.publicKey);
   assert.ok(account.result.eq(new anchor.BN(5)));
  })

  it('subs 2 numbers', async()=>{
    await program.rpc.subtract(new anchor.BN(3), new anchor.BN(2),{
     accounts:{
       calculator: calculator.publicKey
     }
    })
    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(1)));
   })

   it('multiply 2 numbers', async()=>{
    await program.rpc.multiply(new anchor.BN(3), new anchor.BN(2),{
     accounts:{
       calculator: calculator.publicKey
     }
    })
    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(6)));
   })

   it('divide 2 numbers', async()=>{
    await program.rpc.divide(new anchor.BN(7), new anchor.BN(2),{
     accounts:{
       calculator: calculator.publicKey
     }
    })
    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(3)));
    assert.ok(account.remainder.eq(new anchor.BN(1)));
   })
})