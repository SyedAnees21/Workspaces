const mqtt = require('mqtt')
var client = mqtt.connect('mqtt://broker.hivemq.com')

var topic = 'Temprature'

client.on('connect', ()=>{
    setInterval(()=>{
        var random = Math.random()*50
        console.log(random)
        if (random < 30) {
            client.publish(topic,random.toString())
        }
    },1000)
})