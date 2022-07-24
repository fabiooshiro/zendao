import React, { FC } from 'react';
import { Theme } from './Theme';
import WalletContext from './contexts/WalletContext';
import {
    BrowserRouter as Router,
    Routes,
    Route,
} from "react-router-dom";
import {
    Container,
} from '@mui/material';
import { WalletMultiButton } from '@solana/wallet-adapter-material-ui';
import { Validation } from './pages/Validation';
import { Landing } from './pages/Landing';
import { Telegram } from './pages/Telegram';
import { InitDAO } from './pages/InitDAO';
import { clusterApiUrl, Connection } from '@solana/web3.js';
import { ZendaoService } from './services/ZendaoService';

const commitment = 'processed'
const url = new URL(window.location.href)
const network = clusterApiUrl(url.searchParams.get('cluster') as any || 'mainnet-beta')
const connection = new Connection(network, commitment)

ZendaoService.setConnection(connection)

export const App: FC = () => {
    return (
        <Theme>
            <WalletContext>
                <Router>
                    <Container
                        style={{ marginTop: '40px' }}
                        maxWidth='xl'
                    >
                        <WalletMultiButton style={{ marginTop: "4px" }} />
                        <Routes>
                            <Route path='/' element={<Landing />} />
                            <Route path='/validation/:mint' element={<Validation />} />
                            <Route path='/dao/:daoSlug/telegram' element={<Telegram />} />
                            <Route path='/dao/:daoSlug/init' element={<InitDAO />} />
                        </Routes>
                    </Container>
                </Router>
            </WalletContext>
        </Theme>
    );
};
