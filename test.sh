#/bin/sh
function pause {
    read -n 120 -p "Press 'Enter' to continue..." ; echo " "
}


./build.sh
echo 12345678901234567890123456>test.fic
echo abcdefghijklmnopqrstuvwxyz>>test.fic
echo ABCDEFGHIJKLMNOPQRSTUVWZXYZ>>test.fic
./target/debug/cut -c 10-15 test.fic
echo expected: 
echo 12345
echo klmno
echo KLMNO
pause
