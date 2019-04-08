const express = require("express");
const WebSocket = require("ws");
const http_server = express();
http_server.use(express.static(__dirname + "/../client"));

const ws_server = new WebSocket.Server({ port: 3001 });

ws_server.on("connection", connection => {
  console.log("Opened a connection");

  connection.on("message", message => {
    console.log("message received from a client: " + message);

    ws_server.clients.forEach(client => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(message);
      }
    });
  });

  connection.on("close", () => {
    console.log("Closed a connection");
  });

  connection.on("error", error => {
    console.error("Error: " + error.message);
  });
});

//Start the web server serving the WebSocket client
//Open http://localhost:3000 in a web browser
const server = http_server.listen(3000, () => {
  const host = server.address().address;
  const port = server.address().port;
  console.log("Listening to host http://localhost:3000/", host, port);
});
