# logps
A tool to print the log per seconds.It cannot work alone,it need to be after `tail -f `.

# Build
```
cargo build
```
# Usage
```
tail -f /var/log/yourlog | /path/to/logps
```

then it will print
```
logps: 2373503 [1s]
logps: 2330721 [1s]
logps: 2349387 [1s]
logps: 2309434 [1s]
logps: 2305851 [1s]
```
