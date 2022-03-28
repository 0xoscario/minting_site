
const fs = require('fs')

require("dotenv").config();
require('dotenv').config({ path: '../.env' })
const info = require("./info.json");


function main() {
    let a = []
    for(let [key,value] of Object.entries(info)){
       if(value > 5) {
           a.push({
               address : key,
               number : value
           })
       }
      }

    fs.writeFileSync("topholder.json", JSON.stringify(a))

}

main()

