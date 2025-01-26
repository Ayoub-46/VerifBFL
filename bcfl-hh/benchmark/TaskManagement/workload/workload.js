/*const Web3 = require('web3');
let web3 = new Web3(this.nodeUrl);*/
let fs = require('fs');


function generateNewAccount(){
    let address = web3.eth.accounts.create()
    return address.address
}

function makeRandomString(length) {
    var result           = '';
    var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    var charactersLength = characters.length;
    for ( var i = 0; i < length; i++ ) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
}
function getRandomInt(min, max) {
    const randomDecimal = Math.random();
    const randomInt = Math.floor(randomDecimal * (max - min + 1)) + min;
    return randomInt;
}


/*function write(path , newData){
    var data = fs.readFileSync(path);
    var data= JSON.parse(data);
    data.push(newData);
    fs.writeFile(path, JSON.stringify(data, null), err => {
        if(err) throw err;
        console.log("New test data generated !");
    });  
}*/

function write(path, newData) {
    let data = [];

    // Check if the file exists and is not empty
    if (fs.existsSync(path) && fs.statSync(path).size > 0) {
        try {
            // Read and parse existing data
            data = JSON.parse(fs.readFileSync(path, 'utf8'));
        } catch (e) {
            // If there is an error parsing, log it and initialize data as empty array
            console.error('Error parsing JSON from file:', e);
            data = [];
        }
    }

    // Append new data and write to file
    data.push(...newData);
    fs.writeFileSync(path, JSON.stringify(data, null, 2), 'utf8');
    console.log("New test data generated!");
}


let workload = [];

function generateWorkload(n){
    for ( var i = 0; i < n; i++ ) {
        let args = {
            _taskId: makeRandomString(32),
            _initialModel: makeRandomString(32),
            _reward: getRandomInt(10000000001, 500000000000),
            _totalRounds: getRandomInt(1,5)}
        workload.push(args);
    }
    write("./workload.json",workload)
}


generateWorkload(2)


