// The Licensed Work is (c) 2023 ChainSafe
// Code: https://github.com/ChainSafe/Spectre
// SPDX-License-Identifier: LGPL-3.0-only

pragma solidity ^0.8.0;

import "forge-std/Script.sol";

import {ProofOfExploit} from "../src/PoE.sol";

contract Deploy is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("ANVIL_PRIVATE_KEY");
        bytes32 verifierKey = vm.envBytes32("VERIFIER_KEY");


        vm.startBroadcast(deployerPrivateKey);

        new ProofOfExploit(verifierKey);
        
        vm.stopBroadcast();
    }
}
