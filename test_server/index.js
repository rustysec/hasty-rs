const https = require('https')
const express = require('express')
const fs = require('fs')
const app = express()
const morgan = require('morgan')
const bodyParser = require('body-parser')

var options = {
  pfx: fs.readFileSync('./https.pfx'),
  passphrase: ''
};

app.use(new morgan())
app.use(bodyParser.raw({
  inflate: true,
  type: '*/*'
}))

app.get('/done', function (req, res) {
  res.send();
  process.exit(0)
})

app.get('/basic_get', function (req, res) {
  res.send('success')
})

app.post('/basic_post', function (req, res) {
  console.log('got a post: ' + req.body)
  var payload = {
    headers: req.headers,
    bodyLength: req.body.length
  }
  res.send(payload)
});

app.post('/basic_409', function(req, res) {
    res.status(409).send()
});

app.listen(3000, function () {
  console.log('hasty-rs test server listening on 3000!')
})

https.createServer(options, app).listen(3001, function () {
  console.log('hasty-rs test tls server listening on 3001')
})
