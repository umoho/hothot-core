# `Hot-hot`

* Just run it in command line
* It will create threads and do calculations
* Remember to close the program else it will keep running then your machine will overheat
* **Use Ctrl-C to stop**

## Options
#### Use --config to set the config file
* File type is toml
* It can also work with -c
* Default config will be use if no config file is set or has something wrong...

#### Default config like following
```toml
# settings
method = "md5"
threads = -1
```
So it can also work without config file

## On linux
```
./hothot-core
```
or
```
./hothot-core -c config.toml
```

## On windows
```
.\hothot-core.exe
```
