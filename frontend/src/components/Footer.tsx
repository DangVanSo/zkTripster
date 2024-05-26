import React from 'react';
import Box from '@mui/material/Box';
import styled from "@emotion/styled";
import Link from "@mui/material/Link";

const Footer: React.FC = () => {
    return (
        <StyledBox component="footer" sx={{py: 3, px: 2, mt: 'auto', backgroundColor: '#1976d2'}}>
            <Link href='https://github.com/tripster-hackers/zkTripster'>

            <Container>
                <RunningText>
                    Ethical Hacking | Visit our Github
                </RunningText>
            </Container>
            </Link>

        </StyledBox>
    );
};

const RunningText = styled(Box)`
    display: inline-block;
    white-space: nowrap;
    overflow: hidden;
    box-sizing: border-box;
    animation: marquee 10s linear infinite;
    text-transform: none;

    @keyframes marquee {
        0% {
            transform: translateX(-100%);
        }
        100% {
            transform: translateX(100%);
        }
    }
`;

const Container = styled(Box)`
    color: white;
    width: 100%;
    overflow: hidden;
    white-space: nowrap;
    box-sizing: border-box;
    max-width: 300px;
    border: 1px solid white;
    border-radius: 20px;
    text-transform: uppercase;
    background: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
    
    &:hover {
        color: black;
        background: white;
    }
`;


const StyledBox = styled(Box)`
    display: flex;
    justify-content: flex-end;
    background-color: transparent;
    z-index: 5;
    
    a {
        text-transform: none;
    }
`;


export default Footer;
