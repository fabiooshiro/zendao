import { WalletContextState } from "@solana/wallet-adapter-react";
import bs58 from 'bs58';
import tweetnacl from 'tweetnacl';
import { Metadata, MetadataData } from '@metaplex-foundation/mpl-token-metadata'
import { AccountInfo, clusterApiUrl, Connection, PublicKey } from '@solana/web3.js';
import axios from "axios";
import dayjs from "dayjs";

const SPL_PUBLIC_KEY = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA')
let connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

let cachedMetadata: Record<string, any> = {}

export interface NFT {
    mint: string
    name: string
    description: string
    symbol: string
    image: string
    creators: string[]
    attributes: any
    owner: string,
    raw: any
}

export default class CalindraWallet {
    constructor(private wallet: WalletContextState) {
        // nope
    }

    async loadNFTs() {
        const publicKey = this.wallet.publicKey
        if (!publicKey) {
            console.log('Sem publicKey')
            return []
        }
        const rawTokenAccountResult = await connection.getParsedTokenAccountsByOwner(publicKey, {
            programId: SPL_PUBLIC_KEY
        })
        const mayNFTs = rawTokenAccountResult.value.filter(rta => {
            const justOne = rta.account.data.parsed.info.tokenAmount.amount === '1'
            return justOne
        })
        const metadatas = await Promise.all(mayNFTs.map(rta => {
            return this.loadMetadata(rta.account.data.parsed.info.mint)
        }))
        return metadatas.filter(raw => !!raw).map(raw => {
            const nft = this.parseNFT(raw.info, raw.data, raw.metadata)
            nft.owner = publicKey.toBase58()
            return nft
        })
    }

    async loadMetadata(mint: string) {
        const cached = cachedMetadata[mint]
        if (cached) {
            return cached
        }
        const editionMint = new PublicKey(mint)
        const pda = await Metadata.getPDA(editionMint)
        const info = await connection.getAccountInfo(pda)
        const data = MetadataData.deserialize(info!.data)
        const cacheByMinute = dayjs().format('YYYYMMDDHHmm')
        const metadata = await axios.get(data.data.uri, { params: { t: cacheByMinute } }).then(r => r.data)
        return cachedMetadata[mint] = { info, data, metadata }
    }

    static invalidateMetadataCached(mint: string) {
        delete cachedMetadata[mint]
    }

    private parseNFT(info: AccountInfo<Buffer> | null, data: any, metadata: any) {
        let attributes: any = {}
        if (metadata.attributes) {
            metadata.attributes.forEach((attr: any) => {
                attributes[attr.trait_type] = attr.value
            })
        }
        const nft: NFT = {
            owner: '',
            name: metadata.name,
            symbol: metadata.symbol,
            description: metadata.description,
            image: metadata.image,
            mint: data.mint,
            creators: data.data.creators.map((item: any) => item.address),
            attributes,
            raw: { info, data, metadata }
        }
        return nft
    }

    /**
     * Assina e devolve a assinatura em base58
     */
    async signMessage(content: string) {
        const uint8array = new TextEncoder().encode(content);
        if (!this.wallet.signMessage) {
            throw new Error('Metodo signMessage não suportado')
        }
        const signature = await this.wallet.signMessage(uint8array)
        console.log({ signature })
        return bs58.encode(signature)
    }

    /**
     * Passar a assinatura e o publicKey em base58
     */
    verify(content: string, signature: string, publicKey: string) {
        const uint8array = new TextEncoder().encode(content);
        const pubKey = bs58.decode(publicKey)
        const u8signature = bs58.decode(signature)
        return tweetnacl.sign.detached.verify(uint8array, u8signature, pubKey)
    }
}
