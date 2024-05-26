import React from 'react';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import styled from "@emotion/styled";

const Home: React.FC = () => {
    return (
        <StyledContainer>
            <Box my={4}>
                <Typography letterSpacing="1.5px" lineHeight="1.4" fontSize={"58px"} fontFamily={'"Press Start 2P",system-ui'} variant="h4" component="h1"
                            gutterBottom>
                    <Box mb="16px">Time</Box>
                    <Box mb="16px" ml="150px" color="rgba(255, 255, 255, 54%)">Release</Box>
                    <Box mb="16px">Incentive Platform </Box>
                    <Box ml="385px" mb="32px">for</Box>
                    <FlickerBoxStyled>Security Threads</FlickerBoxStyled>
                    <Box ml="190px" mb="16px" mt={"24px"}>
                        Ethical
                        <Typography fontSize={"58px"}
                                    ml={"16px"}
                                    fontFamily={'"Press Start 2P",system-ui'}
                                    display={"inline-block"}
                                    color="rgba(255, 255, 255, 54%)">Reporting
                        </Typography>
                    </Box>
                </Typography>
            </Box>
        </StyledContainer>
    );
};

const StyledContainer = styled(Container)`
    z-index: 5;
    font-family: "Press Start 2P", system-ui;
    font-weight: 400;
    font-style: normal;
    padding: 64px 0;
`

const FlickerBoxStyled = styled(Box)`
    background-image: url(https://uploads-ssl.webflow.com/61c181d8f24ef9de8bdb5e4c/61f88714dc887d15e25d9cb5_Spark2.gif);
    background-position: 50% 50%;
    background-size: 650px;
    font-size: 58px;
    background-repeat: no-repeat;
    background-attachment: scroll;
    font-family: "Press Start 2P", system-ui;
    line-height: .5;
    margin: 0 0 32px 190px;
    display: inline;
`


export default Home;
