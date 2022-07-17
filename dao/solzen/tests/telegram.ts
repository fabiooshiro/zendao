import * as anchor from "@project-serum/anchor";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import Factory from "./Factory";

describe("solzen", () => {
	xit("smoke create the DAO", async () => {
		await Factory.createDAO()
	})

	describe('validateTelegramUser()', () => {
		it('should block token account with wrong owner', async () => {
			const { mint, payer, mintAuthority, daoPubkey, connection } = await Factory.createDAO()
			const program = await Factory.programPaidBy(payer)
			const Keypair = anchor.web3.Keypair;
			const wrong = Keypair.generate();
			
			const tokenAccount = await getOrCreateAssociatedTokenAccount(
				connection,
				payer,
				mint,
				wrong.publicKey,
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

			const telegramUserId = new anchor.BN("1234");
			const [telegramUser, _bump2] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('telegram_user'),
				daoPubkey.toBuffer(),
				telegramUserId.toArray("le", 8),
			], program.programId);

			const error = await program.methods
				.validateTelegramUser(telegramUserId)
				.accounts({
					telegramUser,
					tokenAccount: tokenAccount.address,
					zendao: daoPubkey
				})
				.rpc()
				.catch(e => e)
			expect(error.message).to.contains('WrongOwner')
		})

		it("should block user with few tokens", async () => {
			const { mint, payer, mintAuthority, daoPubkey, connection } = await Factory.createDAO()
			const program = await Factory.programPaidBy(payer)
			
			const tokenAccount = await getOrCreateAssociatedTokenAccount(
				connection,
				payer,
				mint,
				payer.publicKey,
			)

			await mintTo(
				connection,
				payer,
				mint,
				tokenAccount.address,
				mintAuthority,
				// 100000000000 // because decimals for the mint are set to 9 
				1
			)

			const telegramUserId = new anchor.BN("1234");
			const [telegramUser, _bump2] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('telegram_user'),
				daoPubkey.toBuffer(),
				telegramUserId.toArray("le", 8),
			], program.programId);

			const error = await program.methods
				.validateTelegramUser(telegramUserId)
				.accounts({
					telegramUser,
					tokenAccount: tokenAccount.address,
					zendao: daoPubkey
				})
				.rpc()
				.catch(e => e)
			expect(error.message).to.contains('InsuficientAmount')
		})

		it("should validate the Telegram User", async () => {
			const { mint, payer, mintAuthority, daoPubkey, connection } = await Factory.createDAO()
			const program = await Factory.programPaidBy(payer)
			
			const tokenAccount = await getOrCreateAssociatedTokenAccount(
				connection,
				payer,
				mint,
				payer.publicKey,
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

			const telegramUserId = new anchor.BN("1234");
			const [telegramUser, _bump2] = await PublicKey.findProgramAddress([
				anchor.utils.bytes.utf8.encode('telegram_user'),
				daoPubkey.toBuffer(),
				telegramUserId.toArray("le", 8),
			], program.programId);

			const tx = await program.methods
				.validateTelegramUser(telegramUserId)
				.accounts({
					telegramUser,
					tokenAccount: tokenAccount.address,
					zendao: daoPubkey
				})
				.rpc()
			console.log("Your transaction signature", tx);
			const telUserAcc = await program.account.telegramUser.fetch(telegramUser)
			expect(telUserAcc.id.toString()).to.eq(telegramUserId.toString())
			expect(telUserAcc.dao.toString()).to.eq(daoPubkey.toString())
		})
	})

});

