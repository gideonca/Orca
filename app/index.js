const net = require("net");
const { RunCommand } = require("./parser.js");

// Create a TCP server that listens for connections
// and logs received data to the console
console.log("Starting TCP server...");
const server = net.createServer((connection) => {
    connection.forEach((data) => {
        console.log("Received data:", data.toString());
        connection.write(RunCommand(data.toString()));
    });
});

// Start server on port 6379 and bind to localhost
console.log("Server listening...");
server.listen(8080, "127.0.0.1");