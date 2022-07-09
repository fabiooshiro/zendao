import { css } from "@emotion/react";
const backgroundImage = require('../assets/background.jpg');

export const GlobalStyles = css`
    /* 
    CSS RESET
    */
    *,
    *::before,
    *::after {
        box-sizing: border-box;
    }

    /* Remove default margin */
    body,
    h1,
    h2,
    h3,
    h4,
    p,
    figure,
    blockquote,
    dl,
    dd {
        margin: 0;
    }

    /* Remove list styles on ul, ol elements with a list role, which suggests default styling will be removed */
    ul[role='list'],
    ol[role='list'] {
        list-style: none;
    }

    /* Set core root defaults */
    html:focus-within {
        scroll-behavior: smooth;
    }

    /* Set core body defaults */
    body {
        min-height: 100vh;
        text-rendering: optimizeSpeed;
        line-height: 1.5;
    }

    html, body{
        font-family: 'Montserrat', sans-serif;
        font-size: 12px;
        /* background: $bodyBg; */
        color:'#fff';
        /* width: 414px; */
        font-weight: 600;
        margin:  8px auto;
        padding: 4px;
    }

    /* A elements that don't have a class get default styles */
    a:not([class]) {
        text-decoration-skip-ink: auto;
    }

    /* Make images easier to work with */
    img,
    picture {
        max-width: 100%;
        display: block;
    }

    /* Inherit fonts for inputs and buttons */
    input,
    button,
    textarea,
    select {
        font: inherit;
    }

    /* Remove all animations, transitions and smooth scroll for people that prefer not to see them */
    @media (prefers-reduced-motion: reduce) {
        html:focus-within {
            scroll-behavior: auto;
        }

        *,
        *::before,
        *::after {
            animation-duration: 0.01ms !important;
            animation-iteration-count: 1 !important;
            transition-duration: 0.01ms !important;
            scroll-behavior: auto !important;
        }
    }

    *{
        box-sizing: border-box;
    }
    /* 
    END OF CSS RESET
    */


    :root {
        --red: #E52E4D;
        --green: #33cc95;
        --blue: #5429cc;


        --blue-light: #6933ff;

        --text-title: #363f5f;
        --text-body: #969c83;

        --background: #f0f2f5;
        --shape: #ffffff;
    }

    html {
        @media (max-width: 1080px){
            font-size: 93.75%;
        }

        @media (max-width: 720px){
            font-size: 87.5%;
        }
    }

    html,
    body {
        background-color: #303030;
        color: #FFFFFF;
        margin: 0;
        padding: 0;
    }

    body {
        background: var(--background);
        -webkit-font-smoothing: antialiased;
        background-position: center center;
        background-image: url(${backgroundImage});
        /* background-repeat: no-repeat; */
        background-size: cover;
        background-attachment: fixed;
        background-blend-mode: overlay;
        background-color: #303030;
    }

 

    body, input, textarea, button {
        /* font-family: 'Poppins', sans-serif; */
        font-family: "Roboto","Helvetica","Arial",sans-serif;
        font-weight: 400;
    }

    h1, h2, h3, h4, h5, h6, strong {
        font-weight: 600;
    }

    button{
        cursor: pointer;
    }

    [disabled] {
        opacity: 0.6;
        cursor: not-allowed;
    }

    #app {
        min-height: 100vh;
        display: flex;
        justify-content: center;
    }

    a:link,
    :visited {
        color: #FFF;
    }
`