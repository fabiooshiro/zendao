import * as anchor from "@project-serum/anchor";
import { AnchorProvider, Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { createMint } from "@solana/spl-token";
import { LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import { Solzen } from "../target/types/solzen";

let daoSeq = 0
export default class Factory {

    static async createMint() {
        const provider = anchor.AnchorProvider.env()
        const connection = provider.connection
        const Keypair = anchor.web3.Keypair;
        const payer = Keypair.generate();
        const mintAuthority = Keypair.generate();
        const freezeAuthority = Keypair.generate();

        const airdropSignature = await connection.requestAirdrop(
            payer.publicKey,
            LAMPORTS_PER_SOL,
        );

        await connection.confirmTransaction(airdropSignature, "confirmed");
        const mint = await createMint(
            connection,
            payer,
            mintAuthority.publicKey,
            freezeAuthority.publicKey,
            9 // We are using 9 to match the CLI decimal default exactly
        );
        return { mint, payer, mintAuthority, connection }
    }

    static programPaidBy(payer: anchor.web3.Keypair): anchor.Program<Solzen> {
        const provider = anchor.AnchorProvider.env()
        const newProvider = new AnchorProvider(provider.connection, new anchor.Wallet(payer), {});
        const program = anchor.workspace.Solzen;
        return new anchor.Program(program.idl as anchor.Idl, program.programId, newProvider) as Program<Solzen>
    }

    static async createDAO(options?: { daoSlug: string }) {
        const { mint, payer, mintAuthority, connection } = await Factory.createMint();
        const program = await Factory.programPaidBy(payer);
        const daoSlug = options?.daoSlug || ('zendao' + daoSeq++)
        console.log(`criando dao [${daoSlug}]`)
        const [daoPubkey, _bump] = findProgramAddressSync([
            anchor.utils.bytes.utf8.encode('dao'),
            Buffer.from(daoSlug.slice(0, 32)),
        ], program.programId);

        const [userAccount, _bump2] = findProgramAddressSync([
            anchor.utils.bytes.utf8.encode('child'),
            payer.publicKey.toBuffer(),
            daoPubkey.toBuffer(),
        ], program.programId);

        const tx = await program.methods
            .initialize(mint, new anchor.BN(10), daoSlug)
            .accounts({
                zendao: daoPubkey,
                validation: userAccount
            })
            .rpc()
        // console.log("DAO foundation transaction", tx)
        assert(typeof tx === 'string')
        return { daoPubkey, mint, payer, mintAuthority, connection }
    }
}