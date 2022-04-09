# <center>`Hot-hot`</center>

<center>

![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/umoho/hothot-core?include_prereleases&style=plastic)
![GitHub watchers](https://img.shields.io/github/watchers/umoho/hothot-core?style=plastic)

</center>

* Just run it in command line
* It will create threads and do calculations
* Remember to close the program else it will keep running then your machine will overheat
* **Use Ctrl-C to stop**

## Options
#### Use --config to set the config file
* File type is `toml`
* It can also work with -c
* Default config will be use if no config file is set or has something wrong...

#### Default config like following
```toml
# settings
method = "md5"
threads = -1
```
So it can also work without config file

## Usage
### On linux
On your **shell**
```
./hothot-core
```
or
```
./hothot-core -c config.toml
```

### On windows
Recommended to use **Terminal**, because it can support color
```
.\hothot-core.exe
```
## Build
Use Cargo to build
```
cargo build --release
```

