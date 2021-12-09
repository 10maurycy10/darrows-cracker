# darrows-cracker

This is a small rust program to crack a darrows devkey hash.

## running

first build the binary:

``cargo build --release``

then run it:

``mp64 ?l?l?l?d?d | target/release/passwd aaf450a516f3ba08fceaa80687bb5c0147a515a634464517026cfcae3adea8a9``

NOTE: change the hash to the newest Hashed_key.

You will likley have to try other masks 

NOTE: large masks will result in it taking hours to days to run, use ``mp64 ?l?l?l?d?d | target/release/passwd | tee logfile`` to save output to file.

## usage

```
target/release/passwd <hash> [amplifyer] [salt]

Usage of an amplifyer is not required but highly recomended.
Candidates are read from stdin.
If an amplifyer is specifyed, it will be concatinated to candidates before hashing.

all chars in amplifyer are interpreted as a literal exept '?' wich expands to a charset depending on the next char.

l    lower case    qwertyuiopasdfghjklzxcvbnm
u    upper case    QWERTYUIOPASDFGHJKLZXCVBNM
L    letter        qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM
h    hex lower     0123456789abcdef
H    hex upper     0123456789ABCDEF
s    symbols       ~!@#$%^&*()-_=+{[}]|\;':",./<>?
d    digits        0123456789
a    all           qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM0123456789~!@#$%^&*()-_=+{[}]|\;':",./<>?
z    @ZeroTix      /!69

```

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
/?l?l?l?l
?l?l?l?l?l
?l?l?l?l?l?l
/?l?l?l?l?l?l!
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

- ``/bruher`` creator: zero, SHA256(HEX(SHA256($pass:HEX(SHA256($pass:$salt)))) (salt: "fc8877c24d85d246e3234f2dcca3a33a842c32f81b6a8f7f60696da988a1fea4")
