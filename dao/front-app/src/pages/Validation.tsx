import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import * as anchor from "@project-serum/anchor";
import { AnchorProvider } from "@project-serum/anchor";
import idl from '../models/solzen.json';
import { clusterApiUrl, Connection, PublicKey } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';
import { ZendaoService } from '../services/ZendaoService';
import { Amount } from '../components/Amount';
const programID = new PublicKey(idl.metadata.address);

const commitment = 'processed'
const url = new URL(window.location.href)
const network = clusterApiUrl(url.searchParams.get('cluster') as any || 'mainnet-beta')
const childPublicKey = url.searchParams.get('child')
const connection = new Connection(network, commitment)

type ValidationProps = {}

export function Validation({ }: ValidationProps) {
    const { mint: mintUrlParam } = useParams()
    if (!mintUrlParam) {
        return <div>Informe o token</div>
    }
    const [parent, setParent] = useState('')
    const [parentBalance, setParentBalance] = useState('')
    const [userBalance, setUserBalance] = useState('')
    const [childBalance, setChildBalance] = useState('')
    const [minBalance, setMinBalance] = useState('')
    const [daoOwner, setDaoOwner] = useState(false)
    const [daoExist, setExist] = useState(true)
    const [decimals, setDecimals] = useState<number | null>(null)
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
            const program = await ZendaoService.getProgram(wallet)
            const daoData = await program.account.zendao.fetch(await zendao)
            setMinBalance(daoData.minBalance.toString())
        } catch (e) {
            console.log(`Erro ao carregar os dados da DAO`, e)
            setExist(false)
        }
    }

    async function showChildBalance() {
        if (!childPublicKey) {
            return;
        }
        const child = new PublicKey(childPublicKey)
        const tokenAddress = await ZendaoService.findAssociatedTokenAddress(child, mint)
        const balance = await connection.getTokenAccountBalance(tokenAddress)
        setDecimals(balance.value.decimals)
        setChildBalance(balance.value.amount)
    }

    async function showParentBalance() {
        if (!parent) {
            return;
        }
        const parentPubkey = new PublicKey(parent)
        const tokenAddress = await ZendaoService.findAssociatedTokenAddress(parentPubkey, mint)
        const balance = await connection.getTokenAccountBalance(tokenAddress)
        setParentBalance(balance.value.amount)
    }

    async function showUserBalance() {
        if (!wallet?.publicKey) {
            return;
        }
        const parent = new PublicKey(wallet.publicKey)
        const tokenAddress = await ZendaoService.findAssociatedTokenAddress(parent, mint)
        const balance = await connection.getTokenAccountBalance(tokenAddress)
        setUserBalance(balance.value.amount)
    }

    async function showChildStatus() {
        if (!childPublicKey) {
            return;
        }
        try {
            const program = await ZendaoService.getProgram(wallet)
            const child = new PublicKey(childPublicKey)
            const [userAccount, _bump] = await PublicKey.findProgramAddress([
                encoder.encode('child'),
                child.toBuffer(),
                mint.toBuffer(),
            ], program.programId);
            const valAcc = await program.account.validation.fetch(userAccount)
            if (valAcc) {
                setParent(valAcc.parent.toBase58())
            }
        } catch (e) {
            console.log(`Usuario ${childPublicKey} sem validacao`, e)
        }
    }

    useEffect(() => {
        showParentBalance()
    }, [parent])

    useEffect(() => {
        showChildStatus()
        showChildBalance()
        showDAO()
    }, [childPublicKey])

    useEffect(() => {
        showUserBalance()
    }, [wallet?.publicKey])

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
        const program = await ZendaoService.getProgram(wallet)
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
        if (!childPublicKey) {
            console.log('Chave publica nao informada')
            return;
        }
        const program = await ZendaoService.getProgram(wallet)
        const child = new PublicKey(childPublicKey)
        const [userAccount, _bump] = await PublicKey.findProgramAddress([
            encoder.encode('child'),
            child.toBuffer(),
            mint.toBuffer(),
        ], program.programId)
        console.log({ userAccount: userAccount.toBase58(), wallet: wallet.publicKey.toBase58() })
        const tokenAccount = await ZendaoService.findAssociatedTokenAddress(
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
                <div>A chave pública {childPublicKey} que possui o seguinte saldo: <Amount value={childBalance} decimals={decimals} /></div>
                <div>foi validada por: {parent} que possui o seguinte saldo: <Amount value={parentBalance} decimals={decimals} /></div>
            </div>
        ) : (
            <div>Validando: {childPublicKey}</div>
        )}
        <div>O seu saldo é: <Amount value={userBalance} decimals={decimals} /></div>
        <div>O valor mínimo para participar das votações: <Amount value={minBalance} decimals={decimals} /></div>
        <button
            onClick={validate}
        >Validar</button>

        {daoOwner ? (
            <button
                onClick={closeDAO}
            >Fechar a DAO</button>
        ) : null}

        {!daoExist ? (
            <button
                onClick={initializeDAO}
            >
                Iniciar a DAO
            </button>
        ) : null}
    </div>)
}
