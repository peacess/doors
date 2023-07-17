
Setlocal
set batPath=%~dp0
%~d0
cd "%batPath%"

flatc --rust --gen-onefile -o ../rust/dchat/src/idl base.fbs partner.fbs

EndLocal
