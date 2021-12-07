# darrows-cracker

This is a small rust program to crack a darrows devkey hash.

## running

first build the binary:

``cargo build --release``

then run:

``mp64 ?l?l?l?d?d | target/release/passwd``

you will likley have to try other masks 

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

- ``bruh68haha!`` creator: zero, SHA256($pass:$salt) (salt: "some long string to stop stuff form happening") 

- ``nodevcrash`` creator: let, SHA256($pass:$salt) (salt: "some long string to stop stuff form happening")

- ``/bruh`` (not in production)  SHA256(HEX(SHA256($pass:HEX(SHA256($pass:$salt)))) (salt: "fc8877c24d85d246e3234f2dcca3a33a842c32f81b6a8f7f60696da988a1fea4")
