const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");

module.exports = buildModule("TokenContract", (m) => {
    const token = m.contract("TokenContract", [10000000000000]);
  

    return { token };
  });