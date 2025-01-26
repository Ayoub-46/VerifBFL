let fs = require('fs');

var data = fs.readFileSync("./workload.json");
var data= JSON.parse(data);
this.data = data;

for (i=0; i<data.length; i++){
    console.log(data[i].a);
    console.log(data[i].b);

 
    console.log("data length ${}", data.length)
}