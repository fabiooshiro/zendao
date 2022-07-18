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

export function InitDAO() {
    const wallet = useWallet()
    const [token, setToken] = useState('CasshNb6PacBzSwbd5gw8uqoQEjcWxaQ9u9byFApShwT')
    const [quantity, setQuantity] = useState('1000000000')
    const { daoSlug } = useParams()

    async function initDAO() {
        if (!daoSlug) {
            console.log('Sem slug')
            return
        }
        if (!wallet?.publicKey) {
            console.log('Sem wallet')
            return
        }
        console.log('criando', { token, quantity })
        const mint = new PublicKey(token)
        const encoder = new TextEncoder()
        const [daoPubkey, _bump] = await PublicKey.findProgramAddress([
            encoder.encode('dao'),
            Buffer.from(daoSlug.slice(0, 32)),
        ], programID)
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
        console.log("Your transaction signature", tx);
        const daoAcc = await program.account.zendao.fetch(daoPubkey)
    }

    return (<div>
        <div>
            Slug/Name: {daoSlug}
        </div>
        <div>
            Token:
            <input
                value={token}
                onChange={event => {
                    console.log(event.target.value)
                    setToken(event.target.value)
                }}
            />
        </div>
        <div>
            Quantidade:
            <input
                value={quantity}
                onChange={event => {
                    console.log(event.target.value)
                    setQuantity(event.target.value)
                }}
            />
        </div>
        <div>
            <button onClick={() => initDAO()}>Criar</button>
        </div>
    </div>)
}