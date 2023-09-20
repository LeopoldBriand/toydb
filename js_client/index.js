import net from 'net';

class DB_Client {
	constructor(host, port) {
		this.host = host;
		this.port = port;
	}
	async get(index, doc = null) {
		const client = new net.Socket();
		const address = doc ? `${index}:${doc}` : index;
		console.log('GET ' + address);
		return new Promise((res, rej) => {
			client.connect(
				this.port, this.host,
				() => {
					client.write(`GET ${address}`);
				}
			);
			client.on('close', function() {
				rej('closed');
			});
			client.on('data', function(data) {
				client.end();
				res(JSON.parse(data.toString())); 
			});
			client.on('error', err => rej(err));
		});
	}
	async set(index, doc, data) {
		if (data) {
			const client = new net.Socket();
			const address = doc ? `${index}:${doc}` : index;
			const dataString = typeof data === 'object' || Array.isArray(data)
				? `JSON${JSON.stringify(data)}`
				: data;
			console.log(`SET ${address} = ${dataString}`);
			return new Promise((res, rej) => {
				client.connect(
					this.port, this.host,
					() => {
						client.write(`SET ${address} = ${dataString}`);
					}
				);
				client.on('close', function() {
					rej('closed');
				});
				client.on('data', function(data) {
					client.end();
					res(JSON.parse(data.toString())); 
				});
				client.on('error', err => rej(err));
			});
		}
	}
	delete(index, doc = null) {
		const client = new net.Socket();
		const address = doc ? `${index}:${doc}` : index;
		console.log('DELETE ' + address);
		return new Promise((res, rej) => {
			client.connect(
				this.port, this.host,
				() => {
					client.write(`DELETE ${address}`);
				}
			);
			client.on('close', function() {
				rej('closed');
			});
			client.on('data', function(data) {
				client.end();
				res(JSON.parse(data.toString())); 
			});
			client.on('error', err => rej(err));
		});
	}
}

export default DB_Client;