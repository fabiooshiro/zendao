import { Grid } from "@mui/material";
import React from "react";
import { Link } from "react-router-dom";

export function Landing() {
    return (
        <Grid container>
            <Grid item xs={12}>
                <h1>DAO</h1>
                <Link to={'/dao/xpto/init'}>Start</Link>
            </Grid>
        </Grid>
    )
}
