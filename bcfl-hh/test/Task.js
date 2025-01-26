const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Task Contract", function () {
    let Task;
    let taskContract;
    let owner;
    let addr1;
    let addr2;

    beforeEach(async function () {
        // Get the ContractFactory and Signers here.

        Task = await ethers.getContractFactory("Task");
        [owner] = await ethers.getSigners();

        // Deploy the contract before each test
        taskContract = await Task.deploy();
        await taskContract.deployed();
    });

    describe("createTask", function () {
        it("should create a new task with correct parameters", async function () {
            // Example task data
            const _taskId = "zXSgYT7hGRuuaE2oFewrZGYYQ0SPdDvf";
            const _initialModel = "miiODvyXQPaM2f8PNEtjJLxMhQe3NfzL";
            const _reward = 118;
            const _totalRounds = 4;

            // Call the createTask function
            await taskContract.createTask(_taskId, _initialModel, _reward, _totalRounds);

            // Retrieve the task to check if it was created correctly
            const task = await taskContract.tasks(_taskId);

            // Check that the task parameters are correct
            expect(task._taskId).to.equal(_taskId);
            expect(task._initialModel).to.equal(_initialModel);
            expect(task._reward).to.equal(_reward);
            expect(task._totalRounds).to.equal(_totalRounds);
        });

        it("should revert if a task with the same ID already exists", async function () {
            const _taskId = "zXSgYT7hGRuuaE2oFewrZGYYQ0SPdDvf";
            const _initialModel = "miiODvyXQPaM2f8PNEtjJLxMhQe3NfzL";
            const _reward = 118;
            const _totalRounds = 4;

            // First task creation should succeed
            await taskContract.createTask(_taskId, _initialModel, _reward, _totalRounds);

            // Attempting to create the same task again should fail
            await expect(
                taskContract.createTask(_taskId, _initialModel, _reward, _totalRounds)
            ).to.be.revertedWith("Task with this ID already exists");
        });
    });
});