# cmdtest.md

Markdown for testing CLI commands.

## How Does It Work?

Code blocks define the important parts of the test.

### `shell`

The command under test.

````md
```shell
echo 'Hello World'
```
````

This is ran after the `env` and `stdin` blocks, if present.

### `stdout`

This is the expected text sent to `stdout` by the command under test.

````md
```shell
echo 'Hello World'
```

```stdout
Hello World
```
````

### `stderr`

This is the expected text sent to `stderr` by the command under test.

````md
```shell
echo 'Hello World'
```

```stdout
Hello World
```

```stderr

```
````

### `exit`

This is the expected exit code returned by the command under test.

````md
```shell
echo 'Hello World'
```

```stdout
Hello World
```

```stderr

```

```exit
0
```
````

### `env`

Environment variables set for the test. This is in the style of dotenv.

````
```env
NAME=World
```

```shell
echo 'Hello $NAME'
```

```stdout
Hello World
```

```stderr

```

```exit
0
```
````

### `stdin`

````
```stdin
World
```

```env
IN=$(cat)
```

```shell
echo 'Hello $IN'
```

```stdout
Hello World
```

```stderr

```

```exit
0
```
````

## Example

````md
---
test: example
---

## The command to test

```env
NAME=World
```

```shell
echo 'Hello ${NAME}'
```

## Expected `stdout`

```stdout
Hello World
```

## Expected `stderr`

```stderr

```

## Expected exit code

```exit
0
```
````
