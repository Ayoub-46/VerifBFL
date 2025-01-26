// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {AccessManagement} from "./AccessManagement.sol";
import {Verifier} from "./Verifier.sol";

contract TaskContract is AccessManagement {
    IERC20 public token;
    Verifier verifier;

    uint256 public constant NUM_TRAINERS = 5;
    uint256 public constant NUM_AGGREGATORS = 2;
    uint256 public constant MIN_REWARD = 10000000000;
    uint256 public constant MIN_STAKE = 100000000;

    event TaskPublished(string taskId);
    event RoundFinished(string taskId, uint256 round);
    event TrainerSubscribed(string taskId, address trainer);
    event AggregatorSubscribed(string taskId, address aggregator);
    event Staked(address indexed participant, uint256 amount);
    event LocalModelSubmitted(uint256 round, address trainer);
    event GlobalModelSubmitted(uint256 round, address trainer);

    constructor(address tokenAddress, address verifierAddress) {
        token = IERC20(tokenAddress);
        verifier = Verifier(verifierAddress);
    }

    struct Task {
        address publisher;
        address[] trainers;
        address[] aggregators;
        // maps each round i with its global model GM_i
        mapping(uint256 => string) globalModels;
        // maps each round i with its list of local updates
        mapping(uint256 => mapping(address => string)) localUpdates;
        // track number of submissions
        uint256 numberOfLocalSubmissions;
        // to keep track of each trainer's task completion
        mapping(address => uint256) stakedAmount;
        mapping(address => bool) taskCompleted;
        uint256 currentRound;
        uint256 reward;
        uint256 totalRounds;
    }

    // maps each existing task with its hash reference
    mapping(string => Task) public tasks;
    uint256 taskCount;

    function createTask(string memory _taskId, string memory _initialModel, uint256 _reward, uint256 _totalRounds)
        public
        onlyRole(PUBLISHER_ROLE)
    {
        require(_reward >= MIN_REWARD, "reward amount is less than the minimum value");
        require(token.transferFrom(msg.sender, address(this), _reward), "Token transfer failed");
        Task storage task = tasks[_taskId];
        task.publisher = msg.sender;
        task.reward = _reward;
        task.totalRounds = _totalRounds;
        task.currentRound = 0;
        task.globalModels[0] = _initialModel;
        task.numberOfLocalSubmissions = 0;
        emit TaskPublished(_taskId);
    }

    function subscribeAsTrainerToTask(string memory taskId, uint256 stakeValue) 
        public 
        onlyRole(TRAINER_ROLE)
    {
        
        // verify the number total number of trainers registered to tasks[taskId]
        require(tasks[taskId].trainers.length < NUM_AGGREGATORS, "reached max trainers count");
        // verify the non existance of the trainer in the trainers list of tasks[taskid]
        require(!exists(tasks[taskId].trainers, msg.sender), "Can not register twice");

        // freeze the stakeValue using an escrow contract
        require(stakeValue >= MIN_STAKE, "Not enough staked tokens");
        require(token.transferFrom(msg.sender, address(this), stakeValue), "Token transfer failed");

        // tasks[taskId].stakedAmount[sender] = stakeValue;
        emit Staked(msg.sender, stakeValue);

        // add the trainer to the trainers list of tasks[taskId]
         tasks[taskId].trainers.push(msg.sender);

        emit TrainerSubscribed(taskId, msg.sender);
    }

    function subscribeAsAggregatorToTask(string memory taskId, uint256 stakeValue) 
        public 
        
    {
        // verify the number total number of aggregators registered to tasks[taskId]
        require(tasks[taskId].aggregators.length < NUM_AGGREGATORS, "reached max trainers count");
        // verify the non existance of the aggregator in the aggregators list of tasks[taskid]
        require(!exists(tasks[taskId].trainers, msg.sender), "Can not register twice");
        // freeze the stakeValue using an escrow contract
        require(stakeValue >= MIN_STAKE, "Not enough staked tokens");
        require(token.transferFrom(msg.sender, address(this), stakeValue), "Token transfer failed");

        // tasks[taskId].stakedAmount[msg.sender] = stakeValue;
        emit Staked(msg.sender, stakeValue);
        // add the aggregator to the aggregators list of tasks[taskId]
        tasks[taskId].aggregators.push(msg.sender);

        emit AggregatorSubscribed(taskId, msg.sender);
    }


    function submitLocalUpdate(string memory taskId, string memory localUpdate, string memory proof) public {
        uint256 currentRound = tasks[taskId].currentRound;
        // check that the caller is part of the trainers of this task
        require(exists(tasks[taskId].trainers, msg.sender));

        // verify the validity of the submitted proof
        bool res = verifier.verifyTrainingProof(proof);

        require(res, "INVALID PROOF, Blacklisted!");

        tasks[taskId].localUpdates[currentRound][msg.sender] = localUpdate;
        tasks[taskId].numberOfLocalSubmissions++;

        if (tasks[taskId].numberOfLocalSubmissions == tasks[taskId].trainers.length) {
           tasks[taskId].currentRound++;
           emit RoundFinished(taskId, tasks[taskId].currentRound);
        }

        // emit model submition event
        emit LocalModelSubmitted(currentRound, msg.sender);
    }

    function submitGlobalUpdate(string memory taskId, string memory globalUpdate, string memory proof) public {
        uint256 currentRound = tasks[taskId].currentRound;
        // check that the caller is part of the trainers of this task

        require(exists(tasks[taskId].aggregators, msg.sender));

        // check the number of submitted local updates for the current round

        // verifiy the validity of the submitted proof
        bool res = verifier.verifyAggProof(proof);

        require(res, "INVALID PROOF, Blachlisted!");
        // blacklist if proof is not valid

        tasks[taskId].globalModels[currentRound] = globalUpdate;

        // emit model submition event
        emit GlobalModelSubmitted(currentRound, msg.sender);

        // if it's the last local model, emit end of round event
        if (currentRound == tasks[taskId].totalRounds) {
            // distribute rewards
        }
    }

    // utility

    function exists(address[] memory arr, address element) internal pure returns (bool) {
        for (uint256 i = 0; i < arr.length; i++) {
            if (arr[i] == element) {
                return true;
            }
        }
        return false;
    }

    function getTaskPublisher(string memory taskId) public view returns (address){
        return tasks[taskId].publisher;
    }

    function getNumberOfRounds(string memory taskId) public view returns (uint256){
        return tasks[taskId].totalRounds;
    }

    function getCurrentRound(string memory taskId) public view returns (uint256){
        return tasks[taskId].currentRound;
    }
}
