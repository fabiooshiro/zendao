import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { ZendaoService } from '../services/ZendaoService';
import { Amount } from '../components/Amount';
import { useWallet } from '@solana/wallet-adapter-react';
import * as anchor from "@project-serum/anchor";

const url = new URL(window.location.href)

/**
 * Telegram ID
 */
const telegramID = url.searchParams.get('id')

export function Telegram() {
    const { daoSlug } = useParams()
    const [minBalance, setMinBalance] = useState('')
    const [userBalance, setUserBalance] = useState('')
    const [tx, setTx] = useState('')
    const [loading, setLoading] = useState(false)
    const [association, setAssociation] = useState<any>()
    const [decimals, setDecimals] = useState(-1)
    const [preValidation, setPreValidation] = useState(false)
    const [checkBalance, setCheckBalance] = useState(false)
    const [token, setToken] = useState('')
    const wallet = useWallet()

    async function loadDAO(daoSlug) {
        const dao = await ZendaoService.findDao(daoSlug, wallet)
        const supply = await ZendaoService.getConnection().getTokenSupply(dao.token)
        setToken(dao.token.toBase58())
        setDecimals(supply.value.decimals)
        setMinBalance(dao.minBalance.toString())
    }

    async function showUserBalance(wallet) {
        if (!wallet?.publicKey) {
            return;
        }
        if (!daoSlug) {
            return
        }
        try {
            const dao = await ZendaoService.findDao(daoSlug, wallet)
            const tokenAddress = await ZendaoService.findAssociatedTokenAddress(wallet.publicKey, dao.token)
            const balance = await ZendaoService.getConnection().getTokenAccountBalance(tokenAddress)
            setUserBalance(balance.value.amount)
        } catch (e) {
            setUserBalance('')
            if ((e as any).message?.includes('could not find account')) {
                setCheckBalance(true)
            } else {
                console.log(e)
            }
        }
    }

    async function loadAssociation(daoSlug, telegramID, wallet) {
        if (!daoSlug) {
            return
        }
        if (!telegramID) {
            return
        }
        const program = await ZendaoService.getProgram(wallet)
        const [telegramUser, _] = await ZendaoService.findTelegramUserAccount(telegramID, daoSlug, wallet)
        const account = await program.account.telegramUser.fetch(telegramUser)
        setAssociation(account)
    }

    async function validateUser() {
        if (!telegramID) {
            return
        }
        if (!daoSlug) {
            return
        }
        if (!wallet?.publicKey) {
            return
        }
        setLoading(true)
        const program = await ZendaoService.getProgram(wallet)
        const telegramUserId = new anchor.BN(telegramID)
        const [telegramUser, _] = await ZendaoService.findTelegramUserAccount(telegramID, daoSlug, wallet)
        const dao = await ZendaoService.findDao(daoSlug, wallet)
        const [daoPubkey, _bump] = await ZendaoService.findDaoAddress(daoSlug)
        const tokenAccount = await ZendaoService.findAssociatedTokenAddress(wallet.publicKey, dao.token)
        const tx = await program.methods
            .validateTelegramUser(telegramUserId)
            .accounts({
                telegramUser,
                tokenAccount,
                zendao: daoPubkey
            })
            .rpc()
        setTx(tx)
        setLoading(false)
    }

    useEffect(() => {
        loadDAO(daoSlug)
    }, [daoSlug])

    useEffect(() => {
        loadAssociation(daoSlug, telegramID, wallet)
        showUserBalance(wallet)
    }, [telegramID, daoSlug, wallet])

    useEffect(() => {
        if (!minBalance) {
            setPreValidation(false)
            return
        }
        if (!userBalance) {
            setPreValidation(false)
            return
        }
        const min = new anchor.BN(minBalance)
        const amount = new anchor.BN(userBalance)
        if (amount.gte(min)) {
            setPreValidation(true)
        }
    }, [minBalance, userBalance])

    if (association) {
        return (<pre>{JSON.stringify(association, null, 4)}</pre>)
    }

    return (
        <div style={{ paddingTop: '5px' }}>
            <div>DAO: {daoSlug}</div>
            <div>Você está associando a sua chave pública ao usuário do Telegram</div>
            <div>Telegram ID: {telegramID}</div>
            {!wallet?.publicKey ? (
                <div style={{ fontWeight: 800 }}>Para continuar você precisa conectar a sua wallet!</div>
            ) : (
                <>
                    <div>Chave pública: {wallet?.publicKey?.toBase58()}</div>
                    <div>
                        Valor mínimo para participar: <Amount value={minBalance} decimals={decimals} />
                    </div>
                    {checkBalance ? (
                        <div>
                            Verifique o seu saldo
                            oficial na rede Solana <a
                                href={`https://explorer.solana.com/address/${token}`}>
                                {token}
                            </a>
                        </div>
                    ) : (
                        <div>
                            Seu saldo atual: <Amount value={userBalance} decimals={decimals} />
                        </div>
                    )}
                    <div>
                        <button
                            disabled={loading || !preValidation}
                            onClick={() => {
                                validateUser()
                            }}>Associar</button>
                        {tx ? <div>{tx}</div> : null}
                    </div>
                </>
            )}

        </div>
    )
}
