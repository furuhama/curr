## curr

cURL command line wrapper in Rust

## USAGE

First, prepare yaml file for curl execution

```yaml
---
url: https://example.com
method: GET

verbose: false

headers:
  - "Authorization: Basic Xxx"
```

That's all. Just run `curr` command with yaml filename.

```
$ curr sample.yml
```

and you'll get the result.

```
<!doctype html>
<html>
<head>
    <title>Example Domain</title>

    <meta charset="utf-8" />
    <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type="text/css">

      ...

    </style>
</head>

<body>
<div>
    <h1>Example Domain</h1>
    <p>This domain is for use in illustrative examples in documents. You may use this
    domain in literature without prior coordination or asking for permission.</p>
    <p><a href="https://www.iana.org/domains/example">More information...</a></p>
</div>
</body>
</html>
200
```

## Available Options

See [sample.yml](example/sample.yml).
