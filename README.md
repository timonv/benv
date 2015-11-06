# Benv

One of the tenants of the 12factor application is to store configuration in the environment. Often, for production environments, the environment is added
as a .env file during the deploy. [For many languages there are libraries to set this up from inside the application](https://github.com/search?utf8=%E2%9C%93&q=dotenv).

However:

* This introduces a dependency
* Might be in the way when the rest of the application doesn't have to be loaded
* Not very unixy

In a shell, I often solve this by doing `env $(cat /where/my/application/is/.env | sed '/^#/d' | xargs)`. But lets be fair, no one remembers that!

Meet B(etter)env. `benv` simply loads a `.env` file into the environment and starts an application.

## Installation

Via Cargo:

```
$ cargo install benv
```

From source:

```
$ git clone https://github.com/timonv/benv
$ cd benv
$ cargo build --release
$ cp target/release/benv /somewhere/in/your/path
```

## Usage

```
$ benv <dotenv> <program>
```

See `benv --help` for more options.

## Caveats

Current benv has to keep running (there should be little overhead). Using libc to properly daemonize is on the wishlist.

## Contributing

1. Fork
2. Code
3. Test
4. Pull Request :-)
