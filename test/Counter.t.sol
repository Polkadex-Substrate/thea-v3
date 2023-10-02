// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.5.3 <0.9.0;
pragma experimental ABIEncoderV2;

import "forge-std/Test.sol";
import "../src/Counter.sol";

contract CounterTest is Test {
    Counter public counter;

    function setUp() public {
        counter = new Counter();
    }

}
