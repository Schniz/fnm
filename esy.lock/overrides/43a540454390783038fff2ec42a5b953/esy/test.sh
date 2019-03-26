pwd
mkdir _test
cd _test

INCLUDE=$cur__install/include

if which x86_64-w64-mingw32-gcc; then
    CC=x86_64-w64-mingw32-gcc
else
    CC=gcc
fi

echo "Using compiler: $CC"

echo "include..."
ls -a $INCLUDE
echo "lib.."
ls -a $cur__lib

$CC ./../esy/test.c -o ./test.exe -I$INCLUDE -L$cur__lib -lssl -lcrypto

export PATH=$PATH:$cur__bin:$cur__lib
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$cur__lib

echo "Augmented path: $PATH"

echo "Copying binaries..."
cp $cur__bin/*.dll .

echo "Test executable path:"
ls -a .

./test.exe
