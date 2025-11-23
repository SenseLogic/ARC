del /q test.exe
cl arc_curve.cpp arc_vector_3.cpp test.cpp /EHsc /Fe:test.exe
if %ERRORLEVEL% EQU 0 (
    test.exe
)
pause
