#/bin/sh
function pause {
    read -n 120 -p "Press 'Enter' to continue..." ; echo " "
}


./build.sh

export prg=./target/debug/cut

echo 12345678901234567890123456>test.fic
echo abcdefghijklmnopqrstuvwxyz>>test.fic
echo ABCDEFGHIJKLMNOPQRSTUVWZXYZ>>test.fic
$prg -c 10-15 test.fic
echo expected: 
echo 12345
echo klmno
echo KLMNO
pause

$prg -c 40-45 test.fic
echo expected: no data
pause

$prg -c 5-4 test.fic
echo expected: error
pause

echo 1:2:3:4:5:6>test.fic
echo a:b:c:d:e:f>>test.fic
echo A:B:C:D:E:F>>test.fic

$prg -f 2 -d : test.fic
echo expected: 2 b B
pause

$prg -f 8 -d : test.fic
echo expected: nothing
pause


$prg -f 0 -d : test.fic
echo expected: error
pause

$prg -f -10 -d : test.fic
echo expected: error
pause

$prg -f 2 -d ;  test.fic
echo expected: nothing
pause

$prg -f 1 -d ; test.fic
echo expected: full file
pause

