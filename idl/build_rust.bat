
Setlocal
set batPath=%~dp0
%~d0
cd "%batPath%"

flatc --rust -o ../rust/src/idl base.fbs

EndLocal
