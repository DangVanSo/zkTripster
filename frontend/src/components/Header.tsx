import React from 'react';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import {Link as RouterLink} from 'react-router-dom';
import Link, {LinkProps} from '@mui/material/Link';
import styled from '@emotion/styled'
import Button from "@mui/material/Button";
import useMetaMask from "../hooks/useMetamask.ts";
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-expect-error
import Logo from "../assets/logo.svg?react"

const Header: React.FC = () => {
    const {isConnected, connectMetaMask} = useMetaMask();

    return (
        <StyledAppBar position="static" sx={{width: '100%'}}>
            <Toolbar>
                <Typography variant="h6" component="div"
                            sx={{flexGrow: 1, display: 'flex', alignItems: 'center', fontFamily: "Helvetica"}}>
                    zkTripsters
                </Typography>
                <CenterLogo>
                    <StyledLink component={RouterLink} to="/" color="inherit" underline="none" sx={{marginRight: 2}}>
                        <StyledLogo/>
                    </StyledLink>
                </CenterLogo>
                <NavLinks>
                    <StyledLink component={RouterLink} to="/" color="inherit" underline="none" sx={{marginRight: 2}}>
                        Home
                    </StyledLink>
                    <StyledLink component={RouterLink} to="/issue" color="inherit" underline="none"
                                sx={{marginRight: 2}}>
                        Issue
                    </StyledLink>
                    <StyledLink component={RouterLink} to="/hacker-info" color="inherit" underline="none">
                        Hacker info
                    </StyledLink>
                    {!isConnected && (
                        <StyledButton onClick={() => connectMetaMask()} type="button" variant="contained"
                                      size="medium" color="primary">
                            Connect MetaMask
                        </StyledButton>
                    )}
                </NavLinks>
            </Toolbar>
        </StyledAppBar>
    );
};


const StyledAppBar = styled(AppBar)`
    background-color: transparent;
    z-index: 5;
    background-image: none;
    mix-blend-mode: difference;
    border-bottom: 1px solid white;
`;

const StyledLink = styled(Link)<LinkProps & { component: React.ElementType, to?: string }>`
    color: white;
    text-transform: uppercase;
`;

const StyledButton = styled(Button)`
    padding-left: 4px;
    padding-right: 4px;
    margin-left: 20px;
`

const StyledLogo = styled(Logo)`
    height: 64px;
    width: 65px;
    margin-right: 20px;
`

const CenterLogo = styled('div')`
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
`;

const NavLinks = styled('div')`
    display: flex;
    align-items: center;
    margin-left: auto;
`;

export default Header;

