# nsw


[![CircleCI](https://circleci.com/gh/yourgithubhandle/nsw/tree/master.svg?style=svg)](https://circleci.com/gh/yourgithubhandle/nsw/tree/master)


**Contains the following libraries and executables:**

```
nsw@0.0.0
│
├─test/
│   name:    TestNsw.exe
│   main:    TestNsw
│   require: nsw.lib
│
├─library/
│   library name: nsw.lib
│   namespace:    Nsw
│   require:
│
└─executable/
    name:    NswApp.exe
    main:    NswApp
    require: nsw.lib
```

## Developing:

```
npm install -g esy
git clone <this-repo>
esy install
esy build
```

## Running Binary:

After building the project, you can run the main binary that is produced.

```
esy x NswApp.exe 
```

## Running Tests:

```
# Runs the "test" command in `package.json`.
esy test
```
