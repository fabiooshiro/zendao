import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import * as anchor from "@project-serum/anchor";
import { AnchorProvider, Program } from "@project-serum/anchor";
import idl from '../models/solzen.json';
import { clusterApiUrl, Connection, PublicKey } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";

import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { Solzen } from '../models/solzen';

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
const url = new URL(window.location.href)
const network = clusterApiUrl(url.searchParams.get('cluster') as any || 'mainnet-beta')
const publicKey = url.searchParams.get('child')
const connection = new Connection(network, commitment)

type ValidationProps = {
}

export function Validation({ }: ValidationProps) {
    const { mint: mintStr } = useParams()
    if (!mintStr) {
        return <div>Informe o token</div>
    }
    const [parent, setParent] = useState('')
    const [childBalance, setChildBalance] = useState('')
    const wallet = useWallet()

    async function getProvider() {
        const provider = new AnchorProvider(connection, wallet as any, { commitment });
        return provider;
    }

    const mint = new PublicKey(mintStr)//"CasshNb6PacBzSwbd5gw8uqoQEjcWxaQ9u9byFApShwT");
    const encoder = new TextEncoder()
    const [daoPubkey, _bump] = findProgramAddressSync([
        encoder.encode('dao'),
        mint.toBuffer()
    ], programID);

    async function getProgram() {
        const provider = await getProvider()
        anchor.setProvider(provider);
        return new anchor.Program(idl as any, programID, provider) as Program<Solzen>;
    }

    async function showUserBalance() {
        if (!publicKey) {
            return;
        }
        const child = new PublicKey(publicKey)
        const tokenAddress = await findAssociatedTokenAddress(child, mint)
        const balance = await connection.getTokenAccountBalance(tokenAddress)
        const decimals = new anchor.BN(balance.value.decimals)
        const division = new anchor.BN(10).pow(decimals)
        const bnBalance = new anchor.BN(balance.value.amount).divRound(division)
        setChildBalance(bnBalance.toString())
    }

    async function showUserStatus() {
        if (!publicKey) {
            return;
        }
        try {
            const program = await getProgram();
            const child = new PublicKey(publicKey)
            const [userAccount, _bump] = findProgramAddressSync([
                encoder.encode('child'),
                child.toBuffer()
            ], program.programId);
            const valAcc = await program.account.validation.fetch(userAccount)
            console.log(valAcc)
            if (valAcc) {
                setParent(valAcc.parent.toBase58())
            }
        } catch (e) {
            console.log(`Usuario ${publicKey} sem validacao`, e)
        }
    }

    useEffect(() => {
        showUserStatus()
        showUserBalance()
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
        if (!wallet?.publicKey) {
            console.log('Wallet ou chave publica faltando')
            return;
        }
        if (!publicKey) {
            console.log('Chave publica nao informada')
            return;
        }
        const program = await getProgram();
        const child = new PublicKey(publicKey)
        const [userAccount, _bump] = findProgramAddressSync([
            encoder.encode('child'),
            child.toBuffer()
        ], program.programId)
        console.log({ userAccount: userAccount.toBase58(), wallet: wallet.publicKey.toBase58() })
        const tokenAccount = await findAssociatedTokenAddress(
            child,
            mint,
        )
        const [parentValidation, _] = findProgramAddressSync([
            new TextEncoder().encode('child'),
            wallet.publicKey.toBuffer(),
        ], program.programId)
        const tx = await program.methods
            .validateHuman(child)
            .accounts({
                validation: userAccount,
                tokenAccount: tokenAccount,
                zendao: daoPubkey,
                parentValidation,
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
        <div>Saldo: {childBalance}</div>
        <button
            onClick={validate}
        >Validar</button>
    </div>)
}
