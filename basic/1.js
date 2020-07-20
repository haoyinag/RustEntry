const arr = []
const startTimeStamp = new Date().getTime()
for (let q = 0; q < 10000; q++) {
    arr.push({ id: q })
}
const endTimeStamp = new Date().getTime()
const min = endTimeStamp - startTimeStamp
console.log(min)