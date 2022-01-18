@echo off
call build.cmd
echo 12345678901234567890123456>test.fic
echo abcdefghijklmnopqrstuvwxyz>>test.fic
echo ABCDEFGHIJKLMNOPQRSTUVWZXYZ>>test.fic
.\target\debug\cut.exe -c 10-15 test.fic
echo expected: 
echo 12345
echo klmno
echo KLMNO
pause

.\target\debug\cut.exe -c 40-45 test.fic
echo expected: no data
pause

.\target\debug\cut.exe -c 5-4 test.fic
echo expected: error
pause

echo 1:2:3:4:5:6>test.fic
echo a:b:c:d:e:f>>test.fic
echo A:B:C:D:E:F>>test.fic

.\target\debug\cut.exe -f 2 -d : test.fic
echo expected: 2 b B
pause