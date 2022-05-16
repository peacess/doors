
Setlocal
set batPath=%~dp0
%~d0
cd "%batPath%"

:: --gen-onefile 这个参数不能生成 go package name
flatc  -g -o ../go base.fbs partner.fbs

EndLocal
