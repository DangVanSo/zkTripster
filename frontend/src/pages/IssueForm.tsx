import React, {useEffect, useState} from 'react';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import styled from "@emotion/styled";
import {useParams} from "react-router-dom";
import {verifyZkPoex, fetchContractData} from "../utils";
import useMetaMask from "../hooks/useMetamask.ts";
import {purchaseToken, unwatchPurchase, unwatchRedeem} from "../../contracts/src/scripts/api.ts";

const IssueForm: React.FC = () => {
    const {contract_address : contract_address_id} = useParams<{ contract_address: string }>();
    const [zkPoex, setZkPoex] = useState('')
    const [enc, setEnc] = useState('')
    const [purchaseTokenResult, setPurchaseTokenResult] = useState()
    const [verificationResult, setVerificationResult] = useState<string | null>(null);
    const {isConnected, connectMetaMask, account, walletClient} = useMetaMask();

    useEffect(() => {
            if (verifyZkPoex()) {
                setVerificationResult('Valid proof');
            } else {
                setVerificationResult('Invalid proof');
            }
    }, []);


    useEffect(() => {
        if (purchaseTokenResult && isConnected) {
            const purchase = unwatchPurchase()
            const redeem = unwatchRedeem()

            console.log('purchase', purchase)
            console.log('redeem',redeem)
        }
    }, [purchaseTokenResult, isConnected])


    const renderVerificationBox = () => {
        return (
            <VerificationResultBox isValid={verificationResult === 'Valid proof'}>
                <Typography textAlign='center' variant="h6">{verificationResult}</Typography>
            </VerificationResultBox>)
    }

    const renderMetamskConnect =
        () => {
            return (
                <Box marginTop='20px'>
                    {isConnected ? (
                        <Typography variant="body1">
                            Connected: {account}
                        </Typography>
                    ) : (
                        <Button color="inherit" onClick={connectMetaMask}>
                            Connect MetaMask
                        </Button>
                    )}
                </Box>
            )
        }

    console.log('purchaseTokenResult', purchaseTokenResult)

    return (
        <Container>
            <StyledBox my={4}>
                <Typography variant="h4" component="h1" gutterBottom>
                    Receive the vulnerability report
                </Typography>
                {verificationResult ? (
                    <>
                        {renderVerificationBox()}
                        {renderMetamskConnect()}
                        {isConnected && (
                            <Button type='button'
                                    variant="contained"
                                    size="medium"
                                    color="primary"
                                    onClick={async () => {
                                        if (walletClient) {
                                            const purchaseTokenResult = await purchaseToken(walletClient, contract_address_id)
                                            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                                            // @ts-expect-error
                                            setPurchaseTokenResult(purchaseTokenResult)
                                        }
                                    }}>
                                Deposit Tokens
                            </Button>
                        )}
                    </>
                ) : (verificationResult && renderVerificationBox())}
                {purchaseTokenResult && <Box>Transaction hash: ${purchaseTokenResult}</Box>}
            </StyledBox>
        </Container>
    )
        ;
};

const StyledBox = styled(Box)`
    border: 1px solid white;
    padding: 20px;
    background: rgba(0, 0, 0, 70%);
    margin-top: 40px;
`


const VerificationResultBox = styled(Box)<{ isValid: boolean }>`
    margin-top: 20px;
    padding: 10px;
    border: 1px solid ${props => (props.isValid ? 'green' : 'red')};
    background-color: ${props => (props.isValid ? 'rgba(0, 255, 0, 0.1)' : 'rgba(255, 0, 0, 0.1)')};
`;

export default IssueForm;
