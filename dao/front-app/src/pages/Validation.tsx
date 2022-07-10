import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import * as anchor from "@project-serum/anchor";
import { AnchorProvider, Program } from "@project-serum/anchor";
import idl from '../models/solzen.json';
import { clusterApiUrl, Connection, PublicKey } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';

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

type ValidationProps = {}

export function Validation({ }: ValidationProps) {
    const { mint: mintUrlParam } = useParams()
    if (!mintUrlParam) {
        return <div>Informe o token</div>
    }
    const [parent, setParent] = useState('')
    const [childBalance, setChildBalance] = useState('')
    const [minBalance, setMinBalance] = useState('')
    const wallet = useWallet()

    async function getProvider() {
        const provider = new AnchorProvider(connection, wallet as any, { commitment });
        return provider;
    }

    const mint = new PublicKey(mintUrlParam)
    const encoder = new TextEncoder()
    const zendao = new Promise<PublicKey>((resolve, reject) => {
        PublicKey.findProgramAddress([
            encoder.encode('dao'),
            mint.toBuffer(),
        ], programID).then(data => {
            resolve(data[0])
        }).catch(reject)
    })

    async function showDAO() {
        try {
            const program = await getProgram()
            const daoData = await program.account.zendao.fetch(await zendao)
            setMinBalance(daoData.minBalance.toString())
        } catch (e) {
            console.log(`Erro ao carregar os dados da DAO`, e)
        }
    }

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
            const [userAccount, _bump] = await PublicKey.findProgramAddress([
                encoder.encode('child'),
                child.toBuffer(),
                mint.toBuffer(),
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
        showDAO()
    }, [publicKey])

    async function initializeDAO() {
        if (!wallet?.publicKey) {
            return;
        }
        const provider = await getProvider()
        anchor.setProvider(provider);
        const program = new anchor.Program(idl as any, programID, provider);
        const [userAccount, _bump2] = await PublicKey.findProgramAddress([
            anchor.utils.bytes.utf8.encode('child'),
            wallet.publicKey.toBuffer(),
            mint.toBuffer(),
        ], program.programId);
        const tx = await program.methods
            .initialize(mint, new anchor.BN(1000))
            .accounts({
                zendao: await zendao,
                validation: userAccount,
            })
            .rpc()
    }

    async function closeDAO() {
        const program = await getProgram();
        const tx = await program.methods
            .closeDao()
            .accounts({
                zendao: await zendao,
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
        const [userAccount, _bump] = await PublicKey.findProgramAddress([
            encoder.encode('child'),
            child.toBuffer(),
            mint.toBuffer(),
        ], program.programId)
        console.log({ userAccount: userAccount.toBase58(), wallet: wallet.publicKey.toBase58() })
        const tokenAccount = await findAssociatedTokenAddress(
            child,
            mint,
        )
        const [parentValidation, _] = await PublicKey.findProgramAddress([
            new TextEncoder().encode('child'),
            wallet.publicKey.toBuffer(),
            mint.toBuffer(),
        ], program.programId)
        const tx = await program.methods
            .validateHuman(child)
            .accounts({
                validation: userAccount,
                tokenAccount: tokenAccount,
                zendao: await zendao,
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
        <div>Valor mínimo, na menor unidade, para participar das votações: {minBalance}</div>
        <button
            onClick={validate}
        >Validar</button>

        <button
            onClick={closeDAO}
        >Fechar a DAO</button>

        <button
            onClick={initializeDAO}
        >
            Iniciar a DAO
        </button>
    </div>)
}
