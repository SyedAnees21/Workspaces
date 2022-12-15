const express = require('express')
const router = express.Router()
const { User } = require('../models/user');
const {auth} = require('../middleware/requireLogin')

router.get('/',(req,res)=>{
    res.send("hello")
})

router.get('/authorization', auth, (req, res) => {
    // console.log("4")
    // console.log(auth)

    res.status(200).json({
        _id: req._id,
        isAuth: true,
        email: req.user.email,
        name: req.user.name,
        lastname: req.user.lastname,
        role: req.user.role
    })
})

router.post('/signup',(req,res)=>{
      const user = new User(req.body)
    
    //const {name,email,password} = user
    if(!user.email || !user.password || !user.name){
        res.status(422).json({error:"please add all the fields"})
    }
    User.findOne({email: user.email})
    .then((already)=>{
        if(already){
            res.status(422).json({error:"Email already exists"})
        }else{
            user.save((err,doc)=>{
                if(err) return res.status(422).json({Success: false, err});
                return res.status(200).json({Success: true, userData:doc});
            })
        }
    })
})

router.post('/login',(req,res)=>{
    
    User.findOne({email:req.body.email},(err,user)=>{
        if(!user){
            return res.json({LoginSuccess:false, message:'Email not found'});
        }
        user.comparePassword(req.body.password,(err,isMatch)=>{
            if(!isMatch){
                return res.json({LoginSuccess:false, message:'Password Incorrect'});
                
            }else {
                user.generateToken((err,user)=>{
                    if(err) return res.status(400).send(err);
                    res.cookie("Auth_Token", user.token)
                    .status(200)
                    .json({
                        LoginSuccess:true,
                         message:"Logged In"
                        })
                })
            }
        })  
    })
})


module.exports = router