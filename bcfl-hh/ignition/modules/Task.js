const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");

module.exports = buildModule("TaskContract", (m) => {
    
    const token = m.contractAt("TokenContract", "0xB1Ebc63eBf9Dab330a02838FF0eF5faB8D4075a1");


    const task = m.contract("TaskContract", [token.address]);
  
    
    return { task };
  });