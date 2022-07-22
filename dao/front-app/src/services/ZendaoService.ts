import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import * as anchor from "@project-serum/anchor";
import { AnchorProvider, Program } from "@project-serum/anchor";
import { Solzen } from '../models/solzen';
import idl from '../models/solzen.json';
import { WalletContextState } from "@solana/wallet-adapter-react";

const programID = new PublicKey(idl.metadata.address);

const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: PublicKey = new PublicKey(
    'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
);
const encoder = new TextEncoder()
const commitment = 'processed'
const url = new URL(window.location.href)
let network = ''
if (url.searchParams.get('cluster') === 'localhost') {
    network = 'http://localhost:8899'
} else {
    network = clusterApiUrl(url.searchParams.get('cluster') as any || 'mainnet-beta')
}
const connection = new Connection(network, commitment)

export class ZendaoService {
    static async findTelegramUserAccount(telegramID: string, daoSlug: string, wallet?: WalletContextState) {
        const telegramUserId = new anchor.BN(telegramID)
        const program = ZendaoService.getProgram(wallet)
        const [daoPubkey, _bump] = await ZendaoService.findDaoAddress(daoSlug)
        return await PublicKey.findProgramAddress([
            anchor.utils.bytes.utf8.encode('telegram_user'),
            daoPubkey.toBuffer(),
            telegramUserId.toArray("le", 8) as any,
        ], program.programId)
    }
    static async findDaoAddress(daoSlug: string) {
        return await PublicKey.findProgramAddress([
            encoder.encode('dao'),
            Buffer.from(daoSlug.slice(0, 32)),
        ], programID)
    }

    static async findDao(daoSlug: string, wallet?: WalletContextState) {
        const program = ZendaoService.getProgram(wallet)
        const [daoPubkey, _bump] = await ZendaoService.findDaoAddress(daoSlug)
        const daoAcc = await program.account.zendao.fetch(daoPubkey)
        return daoAcc
    }

    static getConnection() {
        return connection
    }

    static async findAssociatedTokenAddress(
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

    static async findDaoUserAccount(mint: PublicKey, child: PublicKey, wallet?: WalletContextState) {
        const program = ZendaoService.getProgram(wallet)
        const [userAccount, _bump] = await PublicKey.findProgramAddress([
            encoder.encode('child'),
            child.toBuffer(),
            mint.toBuffer(),
        ], program.programId)
        return userAccount
    }

    static getProgram(wallet?: WalletContextState) {
        const provider = ZendaoService.getProvider(wallet)
        anchor.setProvider(provider);
        return new anchor.Program(idl as any, programID, provider) as Program<Solzen>;
    }

    static getProvider(wallet?: WalletContextState) {
        const provider = new AnchorProvider(connection, wallet as any, { commitment });
        return provider;
    }

    static format(amount: anchor.BN | string, options: { decimals: number, group?: string }) {
        const bn = new anchor.BN(amount)
        const n = bn.toString(10, 1 + options.decimals)
        const dec = n.substring(n.length - options.decimals)
        const int = n.substring(0, n.length - options.decimals).replace(/^0+/g, '') || '0'
        let intFormated = int.replace(/\B(?=(\d{3})+(?!\d))/g, ",");
        if (options.decimals === 0) {
            return intFormated
        }
        return `${intFormated}.${dec}`
    }
}
