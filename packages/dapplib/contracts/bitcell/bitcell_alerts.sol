// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.2;

contract Test {
    // Variables
    uint num1;
    uint num2;
    constructor() public {
        num1 = 0;
        num2 = 0;
    }
    // Function to update variables
    function updateVars(uint a, uint b) public {
        num1 = a;
        num2 = b;
    }
    // Sum values
    function sumValues() public view returns (uint sum) {
        return num1 + num2;
    }
    event LogMessage(string message);
    function doSomething() external {
        // Do something...
        // Log the action
        emit LogMessage("You did something!");
        let message = sumValues();
        emit NewAlert(string memory message);
    }
}

const TestContract = artifacts.require('./Test');

contract New_test ('Test', accounts => {
    let contractInstance;
    beforeEach(async () => {
    contractInstance = await TestContract.new();
    });
    it('should initalize with num1=0 and num2=0', async () => {
        let num1 = await contractInstance.num1();
        let num2 = await contractInstance.num2();
        assert(num1 === 0 && num2 === 0, "Both num1 and num2 should initalize as 0");
    });
    it('should set num1 and num2 correctly when updateVars is called', async () => {
        let n1 = 10;
        let n2 = 20;
        await contractInstance.updateVars(n1, n2);
        let num1 = await contractInstance.num1();
        let num2 = await contractInstance.num2();
        assert(num1 === n1 && num2 === n2, "Both num1 and num2 were not correctly set");
    });
    it('should return correct sum when sumValues is called', async () => {
        let n1 = 10;
        let n2 = 20;
        await contractInstance.updateVars(n1, n2);
        let sum = await contractInstance.sumValues();
        assert(sum === n1 + n2, "The sum of num1 and num2 was not correctly returned");
    });
});