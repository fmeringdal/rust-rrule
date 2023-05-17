const { get_all_date_recurrences } = require('../../pkg/rrule_wasm.js');

const http = require('http');
const url = require('url');
const hostname = '127.0.0.1';
const port = 3000;

const server = http.createServer((req, res) => {
  const queryObject = url.parse(req.url,true).query;
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/plain');

  const data = get_all_date_recurrences("DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3", 100);
  console.log(data);
  res.end(data.toString());
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});