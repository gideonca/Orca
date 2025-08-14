// This function simulates a Redis-like command response for testing purposes.
// It takes a command string and returns a formatted response. 

function RunCommand(command) {
    switch (command.trim().toUpperCase()) {
        case 'PING':
            return 'PONG';
        default:
            return `Unknown command: ${command}`;  
    }
}

module.exports = {
    RunCommand
};
