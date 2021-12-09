# darrows-cracker

This is a small rust program to crack a darrows devkey hash.

## running

you should first edit the main function to update the hash_tgt.

first build the binary:

``cargo build --release``

then run:

``mp64 ?l?l?l?d?d | target/release/passwd``

you will likley have to try other masks 

NOTE: large masks will result in it taking hours to days to run, use ``mp64 ?l?l?l?d?d | target/release/passwd | tee logfile`` to save output to file.

## speed

This is a parralel cpu hasher, on my T480 it gets arround 1.2 MHashes per second acording to ``bench.sh``.

It should exhast the ``?l?l?l?l?l?l?l?l`` [7] key space in 1.4 hours. (on a debain T480 at stock clock)

Note: bench.sh will output the time to try 1M hashes. the format is
```
real [irl time (seconds)]
user [total cpu time]
sys [time spent waitng for os.]
```

NOTE: if it is not using 100% of your cpu, you may need to pass an amplifyer, which is a hashcat style mask, concatinated to password candiates.

## usefull info

example masks:

```
?s?l?l?l
?l?l?d?d
?l?l?l?d?d
?l?l?l
?l?l?l?l
?l?l?l?l?l
?l?l?l?l?l?l
?l?l?l?l?l?l?l
```

all known dev keys:


- ``arrow69`` creator: zero, SHA256

- ``br12`` creator: zero, SHA256

- ``brhad`` creator: zero, SHA256

- ``dev12``creator: let, SHA256($pass:$salt) (salt: "some long string to stop stuff form happening")

- ``bruh69haha!`` creator: zero, SHA256($pass:$salt) (salt: "some long string to stop stuff form happening") 

- ``nodevcrash`` creator: let, SHA256($pass:$salt) (salt: "some long string to stop stuff form happening")

- ``/bruh`` (not in production)  SHA256(HEX(SHA256($pass:HEX(SHA256($pass:$salt)))) (salt: "fc8877c24d85d246e3234f2dcca3a33a842c32f81b6a8f7f60696da988a1fea4")

- ``/bruher`` creator: let, SHA256(HEX(SHA256($pass:HEX(SHA256($pass:$salt)))) (salt: "fc8877c24d85d246e3234f2dcca3a33a842c32f81b6a8f7f60696da988a1fea4")
