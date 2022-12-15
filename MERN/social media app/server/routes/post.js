const express = require('express')
const res = require('express/lib/response')
const router = express.Router()
const mongoose = require('mongoose')
const post_Model = mongoose.model("Post")
const {auth} = require('../middleware/requireLogin')


router.get('/allposts',(req,res)=>{
    post_Model.find().populate("postedBy","_id name").then(posts=>{
        res.json({posts})
    }).catch(err=>{
        console.log(err)
    })
})

router.post('/createpost',auth,(req,res)=>{
    const {title,body} = req.body

    if(!title || !body){
       return res.status(422).json({error:"Kindly fill all the fields"})
    }

    const post = new post_Model({
        title: req.body.title,
        body:  req.body.body,
        postedBy: req.user
    }) 
    post.save().then(result =>{
        res.json({post:result})
    }).catch(err=>{
        console.log(err)
    })
})

router.get('/myposts',auth,(req,res)=>{
    post_Model.find({postedBy: req.user._id}).populate("postedBy","_id name")
    .then(mypost=>{
        res.json({mypost})
    }).catch(err=>{
        console.log(err)
    })
})
module.exports = router 