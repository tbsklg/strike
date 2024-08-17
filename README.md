# Strikes

## Usage
- Add a strike to a user
```bash
strikes strike <user>
```

- List all strikes
```bash
strikes ls
```

## Use locally only
You can use the local client without a remote server.
It will generate a JSON file where the strikes are stored. 
The default path is in your home directory at '.strikes/db.json'.
You can configure a different location by using the '--db-path' argument or by providing a configuration file.
The argument has precedence over the configuration file.

### Configuration file
The configuration file needs to be a yaml file.

```yaml
db_path: /path/to/db.json
```

The following command will create a database (db.json) in the current directory.

```bash
strikes --db-path ./my-db.json strike <user>
```

