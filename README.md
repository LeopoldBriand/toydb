# Toy DB
A pretty straight forward key value database

## Design
Values are stored with a combination of index and document ids. An index can link to multiple docs.
Docs can only stored simple values (Boolean, Number, String) and JSON values

## Query language
There is 3 standards expressions that are prefix with uniques keywords: GET, SET DELETE.

### GET expression 
Format: `GET index:value` or `GET index`.
Fill index and value will return document data.
Fill only index will return index meta data.

### CREATE or UPDATE expression 
Format: `SET index:value = data`.
data accepts core types like `boolean(true|false)`, `number(0123456789)` or `"string"`. 
and JSON data prefixed with JSON: `JSON[]` or `JSON{}`
Warning: updating json objects will merge existing and incomming objects together.

### DELETE expression 
Format: `DELETE index:value` or `DELETE index`.
Fill index and value will delete document data.
Fill only index will delete index and all linked documents.
