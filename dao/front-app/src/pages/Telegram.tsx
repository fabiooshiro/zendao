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

/**
 * User's Public Key
 */
const pubkey = url.searchParams.get('pubkey')

/**
 * Telegram ID
 */
const id = url.searchParams.get('id')
const connection = new Connection(network, commitment)

export function Telegram() {
    return <div></div>
}
