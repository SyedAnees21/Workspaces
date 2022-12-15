const mqtt = require('mqtt')
var client = mqtt.connect('mqtt://broker.hivemq.com')

var topic = 'Temprature'

client.on('connect', ()=>{
    client.subscribe(topic)
    console.log(topic+' has been subscribed')
})

client.on('message', (topic,msg)=>{
    console.log(msg.toString())
})