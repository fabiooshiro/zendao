import { Global } from '@emotion/react';
import { createTheme, StyledEngineProvider, ThemeProvider } from '@mui/material';
import { deepPurple } from '@mui/material/colors';
import { SnackbarProvider } from 'notistack';
import React, { FC, ReactNode } from 'react';
import { GlobalStyles } from './styles/global';

const theme = createTheme({
    palette: {
        mode: 'dark',
        primary: {
            main: deepPurple[700],
            light: deepPurple[300],
            dark: deepPurple[900],
        },
        secondary: {
            main: '#ADBCA5',
            light: '#7E8978',
            dark: '#7E8978',
        },
        error: {
            main: '#FF1D15',
            light: '#FF5852',
            dark: '#BD2C27',
        },
        warning: {
            main: '#F39237',
            light: '#F39C4B',
            dark: '#B87537',
        },
        info: {
            main: '#1B98E0',
            light: '#3FB0F1',
            dark: '#2A88BF',
        },
        success: {
            main: '#3CBA40',
            light: '#71E675b',
            dark: '#267B29',
        }

    },
    components: {
        MuiButtonBase: {
            styleOverrides: {
                root: {
                    justifyContent: 'center',
                },
            },
        },
        MuiButton: {
            styleOverrides: {
                root: {
                    textTransform: 'none',
                    padding: '8px 16px',
                },
                startIcon: {
                    marginRight: 8,
                },
                endIcon: {
                    marginLeft: 8,
                },

            },
        },
    },
});

export const Theme: FC<{ children: ReactNode }> = ({ children }) => {
    return (
        <StyledEngineProvider injectFirst>
            <Global styles={GlobalStyles} />
            <ThemeProvider theme={theme}>
                <SnackbarProvider
                    anchorOrigin={{
                        vertical: 'bottom',
                        horizontal: 'center',
                    }}
                >
                    {children}
                </SnackbarProvider>
            </ThemeProvider>
        </StyledEngineProvider>
    );
};