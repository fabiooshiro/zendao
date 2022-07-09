import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import * as anchor from "@project-serum/anchor";
import idl from '../models/solzen.json';
import { clusterApiUrl, Connection, PublicKey } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";

import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: PublicKey = new PublicKey(
    'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
);
async function findAssociatedTokenAddress(
    walletAddress: PublicKey,
    tokenMintAddress: PublicKey
): Promise<PublicKey> {
    return (await PublicKey.findProgramAddress(
        [
            walletAddress.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            tokenMintAddress.toBuffer(),
        ],
        SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID
    ))[0];
}
const programID = new PublicKey(idl.metadata.address);

const commitment = 'processed'
// const network = "http://127.0.0.1:8899";
const network = clusterApiUrl('devnet')
const connection = new Connection(network, commitment);

type ValidationProps = {
}
export function Validation({ }: ValidationProps) {
    const { publicKey } = useParams()
    const [parent, setParent] = useState('')
    const wallet = useWallet();

    async function getProvider() {
        const provider = new anchor.AnchorProvider(connection, wallet as any, { commitment });
        return provider;
    }

    const mint = new PublicKey("CasshNb6PacBzSwbd5gw8uqoQEjcWxaQ9u9byFApShwT");
    const [daoPubkey, _bump] = findProgramAddressSync([
        anchor.utils.bytes.utf8.encode('dao'),
        mint.toBuffer()
    ], programID);

    async function getProgram() {
        const provider = await getProvider()
        anchor.setProvider(provider);
        return new anchor.Program(idl as any, programID, provider);
    }

    async function showUserStatus() {
        if (!publicKey) {
            return;
        }
        const program = await getProgram();
        const child = new PublicKey(publicKey)
        const [userAccount, _bump] = findProgramAddressSync([
            anchor.utils.bytes.utf8.encode('child'),
            child.toBuffer()
        ], program.programId);
        const valAcc = await program.account.validation.fetch(userAccount)
        console.log(valAcc)
        if (valAcc) {
            setParent(valAcc.parent.toBase58())
        }
    }

    useEffect(() => {
        showUserStatus()
    }, [publicKey])

    async function initialize() {
        const provider = await getProvider()
        anchor.setProvider(provider);
        const program = new anchor.Program(idl as any, programID, provider);
        const tx = await program.methods
            .initialize(mint, new anchor.BN(1))
            .accounts({
                zendao: daoPubkey
            })
            .rpc()
    }

    async function validate() {
        if (!publicKey) {
            console.log('Chave publica nao informada')
            return;
        }
        const program = await getProgram();
        const child = new PublicKey(publicKey)
        const [userAccount, _bump] = findProgramAddressSync([
            anchor.utils.bytes.utf8.encode('child'),
            child.toBuffer()
        ], program.programId);
        console.log({ userAccount: userAccount.toBase58() })
        const tokenAccount = await findAssociatedTokenAddress(
            child,
            mint,
        )
        const tx = await program.methods
            .validateHuman(child)
            .accounts({
                validation: userAccount,
                tokenAccount: tokenAccount,
                zendao: daoPubkey
            })
            .rpc()
    }

    return (<div>
        {parent ? (
            <div>
                <div>A chave pública: {publicKey}</div>
                <div>Foi validada por: {parent}</div>
            </div>
        ) : (
            <div>Validando: {publicKey}</div>
        )}
        <button
            onClick={validate}
        >Validar</button>
    </div>)
}
