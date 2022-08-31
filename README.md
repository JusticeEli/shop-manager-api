# shop manager api
actix-web backend used to communicate with solana network

The [frontend](https://github.com/JusticeEli/ShopManagement/tree/branch_1#readme) communicating with this Rest api is a Native android application written using `Java` and `Kotlin`.

This backend exposes functionality to help communicate with a [solana program](https://github.com/JusticeEli/shop-manager#readme) in the network

## Build
```bash
    $ cargo build
 ```   

## Run the app
```bash
   $ cargo run # by default am running on port 8080,you can change the configurations in .env file
```        

## Run the tests
```bash
# This options and flag enable the tests to be run synchronously and the stdout to be displayed for each test
   $ cargo test -- --test-threads=1 --nocapture 
```
# REST API ENDPOINTS


## Get list of Goods

### Request

`GET /goods/`

  $  curl -i -H 'Accept: application/json' http://localhost:7000/goods/

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 2

    []

## Create a new Good

### Request

`POST /goods/`

   $ curl -i -H 'Accept: application/json' -d 'name=Foo&status=new' http://localhost:7000/goods

### Response

    HTTP/1.1 201 Created
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 201 Created
    Connection: close
    Content-Type: application/json
    Location: /goods/1
    Content-Length: 36

    {"id":1,"name":"Rice","price":150}

## Get a specific Good

### Request

`GET /goods/id`

   $ curl -i -H 'Accept: application/json' http://localhost:7000/goods/1

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 36

     {"id":1,"name":"Rice","price":150}

## Get a non-existent good

### Request

`GET /goods/id`

  $  curl -i -H 'Accept: application/json' http://localhost:7000/goods/9999

### Response

    HTTP/1.1 404 Not Found
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 404 Not Found
    Connection: close
    Content-Type: application/json
    Content-Length: 35

    {"status":404,"reason":"Not found"}

## Update a good

### Request

`PUT /goods/:id`

   $ curl -i -H 'Accept: application/json' -X PUT -d 'name=Foo&status=changed2' http://localhost:7000/goods/1

### Response

    HTTP/1.1 200 OK
    Date: Thu, 24 Feb 2011 12:36:31 GMT
    Status: 200 OK
    Connection: close
    Content-Type: application/json
    Content-Length: 41

     {"id":1,"name":"Rice","price":154}
    
    
    ## Delete a goods

### Request

`DELETE /goods/id`

  $  curl -i -H 'Accept: application/json' -X DELETE http://localhost:7000/goods/1/

### Response

    HTTP/1.1 204 No Content
    Date: Thu, 24 Feb 2011 12:36:32 GMT
    Status: 204 No Content
    Connection: close


## Try to delete same Good again

### Request

`DELETE /goods/id`

   $ curl -i -H 'Accept: application/json' -X DELETE http://localhost:7000/goods/1/

### Response

    HTTP/1.1 404 Not Found
    Date: Thu, 24 Feb 2011 12:36:32 GMT
    Status: 404 Not Found
    Connection: close
    Content-Type: application/json
    Content-Length: 35

    {"status":404,"reason":"Not found"}

## Get deleted goods

### Request

`GET /goods/1`

   $ curl -i -H 'Accept: application/json' http://localhost:7000/goods/1

### Response

    HTTP/1.1 404 Not Found
    Date: Thu, 24 Feb 2011 12:36:33 GMT
    Status: 404 Not Found
    Connection: close
    Content-Type: application/json
    Content-Length: 35

    {"status":404,"reason":"Not found"}
