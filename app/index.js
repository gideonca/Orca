const net = require("net");

// Create a TCP server that listens for connections
// and logs received data to the console
console.log("Starting TCP server...");
const server = net.createServer((connection) => {
    connection.forEach((data) => {
        console.log("Received data:", data.toString());
    });
});

// Start server on port 6379 and bind to localhost
console.log("Server listening...");
server.listen(6379, "127.0.0.1");