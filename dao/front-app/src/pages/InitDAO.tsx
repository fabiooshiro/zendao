import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import * as anchor from "@project-serum/anchor";
import { PublicKey } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';
import { ZendaoService } from '../services/ZendaoService';

export function InitDAO() {
    const wallet = useWallet()
    const [token, setToken] = useState('CasshNb6PacBzSwbd5gw8uqoQEjcWxaQ9u9byFApShwT')
    const [quantity, setQuantity] = useState('1000000000')
    const { daoSlug } = useParams()
    const [tx, setTx] = useState('')
    const [daoAccount, setDaoAccount] = useState<any>()

    useEffect(() => {
        loadDAO(daoSlug)
    }, [daoSlug])

    async function loadDAO(daoSlug) {
        if (!daoSlug) {
            return
        }
        const program = await ZendaoService.getProgram(wallet)
        const [daoPubkey, _bump] = await ZendaoService.findDaoAddress(daoSlug)
        const daoAcc = await program.account.zendao.fetch(daoPubkey)
        setDaoAccount(daoAcc)
    }

    async function initDAO() {
        if (!daoSlug) {
            alert("Sem slug")
            return
        }
        if (!wallet?.publicKey) {
            alert("Conecte a sua wallet")
            return
        }
        const mint = new PublicKey(token)
        const [daoPubkey, _bump] = await ZendaoService.findDaoAddress(daoSlug)
        const program = await ZendaoService.getProgram(wallet)

        const [userAccount, _bump2] = await PublicKey.findProgramAddress([
            anchor.utils.bytes.utf8.encode('child'),
            wallet.publicKey.toBuffer(),
            daoPubkey.toBuffer(),
        ], program.programId);

        const tx = await program.methods
            .initialize(mint, new anchor.BN(1000), daoSlug)
            .accounts({
                zendao: daoPubkey,
                validation: userAccount,
            })
            .rpc()
        setTx(tx)
    }

    if (daoAccount) {
        return (
            <pre>{JSON.stringify(daoAccount, null, 4)}</pre>
        )
    }

    return (
        <div>
            <div>
                Slug/Name: {daoSlug}
            </div>
            <div>
                Token:
                <input
                    value={token}
                    onChange={event => {
                        setToken(event.target.value)
                    }}
                />
            </div>
            <div>
                Quantidade:
                <input
                    value={quantity}
                    onChange={event => {
                        setQuantity(event.target.value)
                    }}
                />
            </div>
            <div>
                <button onClick={() => initDAO()}>Criar</button>
                {tx ? <div>Transaction: {tx}</div> : null}
            </div>
        </div>
    )
}