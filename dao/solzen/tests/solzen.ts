import * as anchor from "@project-serum/anchor";
import { BorshAccountsCoder } from "@project-serum/anchor";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import Factory from "./Factory";

describe("solzen", () => {

	describe('initialize()', () => {
		it("should initialize the DAO", async () => {
			const { mint, payer } = await Factory.createMint();
			const daoSlug = 'the-dao'
			const program = await Factory.programPaidBy(payer)
			const [daoPubkey, _bump] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('dao'),
				Buffer.from(daoSlug.slice(0, 32)),
			], program.programId);

			const [userAccount, _bump2] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('child'),
				payer.publicKey.toBuffer(),
				daoPubkey.toBuffer(),
			], program.programId);

			const tx = await program.methods
				.initialize(mint, new anchor.BN(1000), daoSlug)
				.accounts({
					zendao: daoPubkey,
					validation: userAccount,
				})
				.rpc()
			console.log("Your transaction signature", tx);
			const daoAcc = await program.account.zendao.fetch(daoPubkey)
			expect(daoAcc.token.toBase58()).to.eq(mint.toBase58())
			expect(daoAcc.minBalance.toString()).to.eq("1000")
		})
	})

	describe('close_dao()', async () => {
		it('should close the DAO', async () => {
			const { payer, daoPubkey } = await Factory.createDAO()
			const program = await Factory.programPaidBy(payer)
			const tx = await program.methods
				.closeDao()
				.accounts({
					zendao: daoPubkey,
				})
				.rpc()
			console.log("Your transaction signature", tx);
			const notFoundError = await program.account.zendao.fetch(daoPubkey).catch(e => e)
			expect(notFoundError.message).to.contains('Account does not exist')
		})
	})

	describe('validateHuman()', () => {
		it("should validate an user as human", async () => {
			const { mint, payer, mintAuthority, daoPubkey, connection } = await Factory.createDAO()
			const child = anchor.web3.Keypair.generate()
			const tokenAccount = await getOrCreateAssociatedTokenAccount(
				connection,
				payer,
				mint,
				child.publicKey
			)

			await mintTo(
				connection,
				payer,
				mint,
				tokenAccount.address,
				mintAuthority,
				10
			)

			const program = await Factory.programPaidBy(payer)

			const [userAccount, _bump] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('child'),
				child.publicKey.toBuffer(),
				daoPubkey.toBuffer(),
			], program.programId);

			const [parentValidation, _bump2] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('child'),
				payer.publicKey.toBuffer(),
				daoPubkey.toBuffer(),
			], program.programId);

			const tx = await program.methods
				.validateHuman(child.publicKey)
				.accounts({
					validation: userAccount,
					tokenAccount: tokenAccount.address,
					zendao: daoPubkey,
					parentValidation
				})
				.rpc()

			console.log("Your transaction signature", tx);

			const valAcc = await program.account.validation.fetch(userAccount)
			expect(valAcc.parent.toBase58()).to.eq(payer.publicKey.toBase58())
			expect(valAcc.child.toBase58()).to.eq(child.publicKey.toBase58())
		});

		it("should validate the mint", async () => {
			const { payer, daoPubkey, connection } = await Factory.createDAO()
			const wrongDao = await Factory.createDAO({ daoSlug: 'other-dao' })
			const mintToTest = wrongDao.mint

			const child = anchor.web3.Keypair.generate()
			const tokenAccount = await getOrCreateAssociatedTokenAccount(
				connection,
				wrongDao.payer,
				wrongDao.mint,
				child.publicKey
			)

			await mintTo(
				connection,
				wrongDao.payer,
				mintToTest,
				tokenAccount.address,
				wrongDao.mintAuthority,
				10
			)
			
			const program = await Factory.programPaidBy(payer)

			const [userAccount, _bump] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('child'),
				child.publicKey.toBuffer(),
				daoPubkey.toBuffer(),
			], program.programId);

			const [parentValidation, _bump2] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('child'),
				payer.publicKey.toBuffer(),
				daoPubkey.toBuffer(),
			], program.programId);

			console.log('ok')
			const error = await program.methods
				.validateHuman(child.publicKey)
				.accounts({
					validation: userAccount,
					tokenAccount: tokenAccount.address,
					zendao: daoPubkey,
					parentValidation
				})
				.rpc()
				.catch(e => e)

			expect(error.message).to.contains('WrongToken')

		});

		it("should validate the parent signature", async () => {
			const { mint, payer, daoPubkey, connection } = await Factory.createDAO()
			const program = await Factory.programPaidBy(payer)
			const wrongSigner = anchor.web3.Keypair.generate()
			const child = anchor.web3.Keypair.generate()
			const [userAccount, _bump] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('child'),
				child.publicKey.toBuffer(),
				mint.toBuffer(),
			], program.programId)

			const tokenAccount = await getOrCreateAssociatedTokenAccount(
				connection,
				payer,
				mint,
				child.publicKey
			)

			const [parentValidation, _bump2] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('child'),
				payer.publicKey.toBuffer(),
				daoPubkey.toBuffer(),
			], program.programId);

			const error = await program.methods
				.validateHuman(child.publicKey)
				.accounts({
					validation: userAccount,
					parent: wrongSigner.publicKey,
					tokenAccount: tokenAccount.address,
					zendao: daoPubkey,
					parentValidation
				})
				.rpc()
				.catch(e => e)

			expect(error.message).to.equal('Signature verification failed')
		});

		it("should validate only 1x", async () => {
			const { mint, payer, mintAuthority, daoPubkey, connection } = await Factory.createDAO()
			const child = anchor.web3.Keypair.generate()
			const program = await Factory.programPaidBy(payer)
			const [userAccount, _bump] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('child'),
				child.publicKey.toBuffer(),
				daoPubkey.toBuffer(),
			], program.programId);

			const tokenAccount = await getOrCreateAssociatedTokenAccount(
				connection,
				payer,
				mint,
				child.publicKey
			)

			await mintTo(
				connection,
				payer,
				mint,
				tokenAccount.address,
				mintAuthority,
				// 100000000000 // because decimals for the mint are set to 9 
				10
			)

			const [parentValidation, _bump2] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('child'),
				payer.publicKey.toBuffer(),
				daoPubkey.toBuffer(),
			], program.programId);

			const tx = await program.methods
				.validateHuman(child.publicKey)
				.accounts({
					validation: userAccount,
					tokenAccount: tokenAccount.address,
					zendao: daoPubkey,
					parentValidation
				})
				.rpc()

			await new Promise((resolve) => setTimeout(() => resolve(1), 1000))

			expect(tx).to.be.a('string')
			console.log("Your transaction signature", tx);
			const error = await program.methods
				.validateHuman(child.publicKey)
				.accounts({
					validation: userAccount,
					tokenAccount: tokenAccount.address,
					zendao: daoPubkey,
					parentValidation
				})
				.rpc().catch(e => e)

			expect(error.message).to.equal('failed to send transaction: Transaction simulation failed: Error processing Instruction 0: custom program error: 0x0')
		})

	})

});

