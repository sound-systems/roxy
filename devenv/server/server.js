// server.js
const WebSocket = require('ws');

const port = process.env.PORT || 8080;
const serverName = process.env.SERVER_NAME || 'server';

const wss = new WebSocket.Server({ port }, () => {
    console.log(`${serverName} listening on port ${port}`);
});

wss.on('connection', ws => {
    ws.on('message', message => {
        if (message === 'ping') {
            ws.send('pong');
        }
    });

    const sendAddress = setInterval(() => {
        ws.send(`Server: ${serverName} listening on port ${port}`);
    }, 5000);

    ws.on('close', () => {
        clearInterval(sendAddress);
    });
});
