function calculateDaysBetweenDates(begin, end) {
    var beginDate = new Date(begin);
    var endDate = new Date(end);
    var days = Math.round(endDate - beginDate) / (1000 * 60 * 60 * 24);
    return days;
}

// find all images without alternate text
// and give them a red border
function process() {
    var images = document.getElementsByTagName("img");
    for (var i = 0; i < images.length; i++) {
        if (images[i].alt == "") {
            images[i].style.border = "2px solid red";
        }
    }
}

// Express server on port 3000
var express = require('express');
var app = express();
app.get('/', function (req, res) {
    res.send('Hello World!');
});

// Return the current time
function getCurrentTime() {
    var date = new Date();
    return date.getHours() + ":" + date.getMinutes();
}
