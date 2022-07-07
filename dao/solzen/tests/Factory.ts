import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { createMint } from "@solana/spl-token";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { Solzen } from "../target/types/solzen";

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
        return { mint, payer, mintAuthority }
    }

    static async createDAO() {
        const provider = anchor.AnchorProvider.env()

        anchor.setProvider(provider);

        const program = anchor.workspace.Solzen as Program<Solzen>;
        const { mint, payer, mintAuthority } = await Factory.createMint();

        const [daoPubkey, _bump] = findProgramAddressSync([
            anchor.utils.bytes.utf8.encode('dao'),
            mint.toBuffer()
        ], program.programId);

        const tx = await program.methods
            .initialize(mint, new anchor.BN(10))
            .accounts({
                zendao: daoPubkey
            })
            .rpc()
        return { daoPubkey, mint, payer, mintAuthority }
    }
}