# Javascript Client

## How to use

```js
import DB_Client from 'db';

const client = new DB_Client(host, port);

// Get data or index meta data with GET action
// This is an equivalent of 'GET index' query
await DB_client.get(index);
// This is an equivalent of 'GET index:doc' query
await DB_client.get(index, doc);

// Create or update data with SET action
// This is an equivalent of 'SET index:doc = data' query
await DB_client.set(index, doc, data);

// Delete docs or indices with DELETE action
// This is an equivalent of 'DELETE index' query
await DB_client.delete(index);
// This is an equivalent of 'DELETE index:doc' query
await DB_client.delete(index, doc);
```


