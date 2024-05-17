POST /samples - upload a sample

GET  /samples - get all uploaded and completed samples

GET  /samples/{sample_id} - get a sample

GET  /samples/{sample_id}/blob - get the root file listing for a sample

GET  /samples/{sample_id}/blob/*path - get a specific file or directory for a sample

GET  /samples/{sample_id}/packages/{package_index} - get a package for a sample

POST /samples

```json
{
    "id": "689ff82d00f64d9ecd1728c04628cf4b7a18b35a17d82f3edd6b83c25de73d62",
    "name": "phoenix.edz",
    "size": 1075022,
    "created": "2024-04-12T07:46:58.021704199Z",
    "status": "pending"
}
```

POST /samples

```json
{
    "id": "689ff82d00f64d9ecd1728c04628cf4b7a18b35a17d82f3edd6b83c25de73d62",
    "name": "phoenix.edz",
    "size": 1075022,
    "created": "2024-04-12T07:46:58.021704199Z",
    "status": "running"
}
```

GET  /samples/689ff82d00f64d9ecd1728c04628cf4b7a18b35a17d82f3edd6b83c25de73d62

```json
{
    "id": "689ff82d00f64d9ecd1728c04628cf4b7a18b35a17d82f3edd6b83c25de73d62",
    "name": "phoenix.edz",
    "size": 1075022,
    "created": "2024-04-12T07:46:58.021704199Z",
    "status": "failed"
}
```

GET  /samples/689ff82d00f64d9ecd1728c04628cf4b7a18b35a17d82f3edd6b83c25de73d62

```json
{
    "id": "689ff82d00f64d9ecd1728c04628cf4b7a18b35a17d82f3edd6b83c25de73d62",
    "name": "phoenix.edz",
    "size": 1075022,
    "created": "2024-04-12T07:46:58.021Z",
    "status": "completed",
    "completed": "2024-04-12T07:46:58.197Z",
    "expires": "2024-04-13T07:46:58.197Z",
    "packages": 1
}
```

GET  /samples/689ff82d00f64d9ecd1728c04628cf4b7a18b35a17d82f3edd6b83c25de73d62/packages/0

```json
{
    "index": 0,
    "kind": "part",
    "name": "PXC.2904622",
    "image": "https://rplan.tethy.xyz/samples/689ff82d00f64d9ecd1728c04628cf4b7a18b35a17d82f3edd6b83c25de73d62/blob/items/picture/PXC/77219_4000_int_04.jpg",
    "manufacturer": {
        "fax": "+49 (0)52 35 31 29 99",
        "town": "Blomberg",
        "name1": "Phoenix Contact GmbH & Co. KG",
        "ziptown": "32825",
        "email": "info@phoenixcontact.com",
        "street": "Flachsmarktstraße 8",
        "note": "??_??@www.phoenixcontact.com;",
        "longname": "Phoenix Contact",
        "phone": "+49 (0)52 35 31 20 00",
        "state": "Germany",
        "shortname": "PXC",
        "lastchange": "EDP / 21.11.2019 08:18:50"
    }
}
```

GET  /samples/689ff82d00f64d9ecd1728c04628cf4b7a18b35a17d82f3edd6b83c25de73d62/blob

```json
[
    {
        "name": "manifest.xml",
        "is_dir": false
    },
    {
        "name": "items",
        "is_dir": true
    }
]
```

Link to our Project: [EDZ to AASX Converter](https://rplan.tethy.xyz/upload)
