import React from 'react';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import styled from "@emotion/styled";

const Home: React.FC = () => {
    return (
        <StyledContainer>
            <Box my={4}>
                <Typography variant="h4" component="h1" gutterBottom>
                    Time Release Incentive Platform for Security Threads Ethical Reporting
                </Typography>
            </Box>
        </StyledContainer>
    );
};

const StyledContainer = styled(Container)`
    z-index: 5;
`


export default Home;
