# pause

just some small binary with almost no dependencies, for mostly container tests

## motivation

when you just want to pause a container, to exec into and do some stuff.

easier to use, than to remember sth. like the following line

`while true; sleep 1; done`

## use:

`pause <seconds to pause>`

`pause` 

will pause until you stop it 


if you provide a smaller number than 0 or an empty string, it will run forever as well

pause for forever:

`pause -1` 

`pause ""`

pause for 5 seconds:

`pause 5` 


*the number is an int64, if you need more, let me know ;)*

## build

`cargo build`

## docker 

in case, you don't have rust installed, you can build a docker image, it is multistage, so the build is handled within.

`docker build -t pause:0.1.0 .`

`docker run --rm pause:0.1.0 <opt: args>`

`docker run --rm pause:0.1.0 5` will run for 5 seconds

if you want to reduce the attack vector even more, you can build the binary from the scratch image, statically linked

`docker build -t pause:0.1.0 . -f scratch.Dockerfile`