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
    const [tx, setTx] = useState('')
    const [loading, setLoading] = useState(false)
    const [association, setAssociation] = useState<any>()
    const wallet = useWallet()

    async function loadDAO(daoSlug) {
        const dao = await ZendaoService.findDao(daoSlug, wallet)
        setMinBalance(dao.minBalance.toString())
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
        console.log(account)
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
    }, [telegramID, daoSlug, wallet])

    if (association) {
        return (<pre>{JSON.stringify(association, null, 4)}</pre>)
    }

    return (
        <div style={{ paddingTop: '5px' }}>
            <div>DAO: {daoSlug}</div>
            <div>Associando a sua chave pública ao usuário do Telegram</div>
            <div>Telegram ID: {telegramID}</div>
            <div>Chave pública: {wallet?.publicKey?.toBase58()}</div>
            <div>
                Valor mínimo: <Amount value={minBalance} decimals={9} />
            </div>
            <div>
                <button
                    disabled={loading}
                    onClick={() => {
                        validateUser()
                    }}>Associar</button>
                {tx ? <div>{tx}</div> : null}
            </div>
        </div>
    )
}
