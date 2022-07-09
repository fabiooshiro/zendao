import React, { FC } from 'react';
import { Theme } from './Theme';
import WalletContext from './contexts/WalletContext';
import {
    BrowserRouter as Router,
    Routes,
    Route,
    HashRouter,
} from "react-router-dom";
import {
    Container,
} from '@mui/material';
import { WalletMultiButton } from '@solana/wallet-adapter-material-ui';
import { Validation } from './pages/Validation';
import { Landing } from './pages/Landing';

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
                            <Route path='/validation/:publicKey' element={<Validation />} />
                        </Routes>
                    </Container>
                </Router>
            </WalletContext>
        </Theme>
    );
};
