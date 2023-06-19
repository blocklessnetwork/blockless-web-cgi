# blockless-web-cgi is the demo project to show how the cgi work.

![](blockless.png)

## How to get the template project.

```bash
wget https://github.com/blocklessnetwork/blockless-web-cgi/archive/refs/heads/main.zip
```

## How to build.

Use the follow command you can build the wasi file.

```bash
$ make release
```

## How to run.

Download the blockless runtime.  https://github.com/blocklessnetwork/runtime/releases

Modify the cgi script `cgi-web`

```python
#### this is security for cgi plugin
opts, args = getopt.getopt(sys.argv[1:], '', ['ext_verify'])
verify = {"alias": "cgi-web","description": "this is cgi-web test","is_cgi": True}
is_verify = False
for o, a in opts:
    if o in ('--ext_verify'):
        is_verify = True

if is_verify:
    print(json.dumps(verify))
    sys.exit(0)
#### this is security for cgi plugin#####

print("web-cgi: hello world")
```

Now you can start the server and browse to http://localhost:8000/ to see it in action:

```bash
$ blockless-cli blockless-web-cgi.wasm - --drivers-root-path=$DIRECTORY_OF_CGI_WEB
```



