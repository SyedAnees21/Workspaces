const express = require('express')
const app = express()
const PORT = 5000
const mongoose = require('mongoose')
const bodyParser = require('body-parser');
const cookieParser = require('cookie-parser');
const cors= require('cors')

/* Database Models*/
const { User } = require('./models/user');
const post_Model = require('./models/post');


/*Routers Paths */
const postRoutes = require('./routes/post');
const registerLoginRoutes = require('./routes/auth')

mongoose.connect('mongodb+srv://SyedAnees21:Octuber21@cluster0.vaq8g.mongodb.net/mySecondDatabase?retryWrites=true&w=majority', { useNewUrlParser: true })
    .then(() => console.log('DataBase connected'))
    .catch((err) => console.log(err))


app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());
app.use(cookieParser());
app.use(cors())

app.use(postRoutes)
app.use(registerLoginRoutes)


app.listen(PORT,()=>{
    console.log("server running at: ", PORT)
})
