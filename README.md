# git-store

This is a prototype of an experimental git/github based storage approach for the edtior that is implemented in Rust. It's a simple CLI tool that can easily be wrapped by a node process to expose it as a web API.

## How it works

It's a simple CLI tool with a couple of commands that take/return JSON as a communication format.

### Init file set

Here is how a new file set is created.

`git-store --init --remote="https://github.com/thoughtram-editor/index.git" --push --data="{ "files": { "style.css": "button: { color: red; }", "app.js": "alert('foo');" } }"`

Behind the scenes, it first creates a new uniqie id (`47f6B` in our example) and then runs `git init 47f6B && cd 47f6B`.
It then walks through the file data provided by the `--data` parameter and creates the actual files in the working directory.
After that it runs `git add -A && git commit -m "some message"`.

It then parses the output and returns:

```
{
    "id": "47f6B"
    "repository": "/path/to/47f6B"
    "last_commit": "ac8ada041e9fd887167ec48b5474778d4f413f91"
}
```

If the `--remote` parameter is provided (as in the example call above) it will automatically
set the provided URL as the `origin` remote.

If the `--push` paramter is provided it will automatically run `git push origin master` as last step.

### Update file set

Here is how an existing fileset is updated.

`git-store --update-set="47f6B" --push --data="{ "files": { "style.css": "button: { color: red; }", "app.js": "alert('foo');" } }"`

Behind the scenes it first `cd`s into the `47f6B`.
It then walks through the file data provided by the `--data` parameter and creates/overwrites the actual files in the working directory.
After that it runs `git add -A && git commit -m "some message"`.

It then parses the output and returns:


```
{
    "id": "47f6B"
    "repository": "/path/to/47f6B"
    "last_commit": "ac8ada041e9fd887167ec48b5474778d4f413f91"
}
```

If the `--push` paramter is provided it will automatically run `git push origin master` as last step.
