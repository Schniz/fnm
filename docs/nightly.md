# Working with Node nightly builds

fnm doesn't give you any shorthands to install nightly builds, but you can
do that by manually passing a "dist mirror" with the `--node-dist-mirror` flag. 

## Example usage

Here's an example of installing Node 23.

To get a list of available versions run this:

    fnm --node-dist-mirror https://nodejs.org/download/nightly/ ls-remote

To use and install nightly version run this:

    fnm --node-dist-mirror https://nodejs.org/download/nightly/ use 23
    # or
    fnm --node-dist-mirror https://nodejs.org/download/nightly/ use v23.0.0-nightly202407253de7a4c374

Once you installed it you would be able to see that version in you list:

    fnm ls

And use it without providing `--node-dist-mirror` flag:

    fnm use 23
