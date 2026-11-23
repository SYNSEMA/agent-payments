@echo off
REM intent-proof · demo en un comando
REM   1. corre los tests del proof
REM   2. verifica el bundle honesto (offline, sin red)
REM   3. verifica el bundle manipulado
setlocal
cd /d "%~dp0"

echo.
echo === 1. TESTS DEL PROOF ===============================================
synsema test src\proof.test.syn
if errorlevel 1 exit /b 1

echo.
echo === 2. VERIFICAR EL BUNDLE HONESTO (offline) ==========================
synsema run src\verify.syn -- proof-bundle.json
if errorlevel 1 goto :bad

echo.
echo === 3. VERIFICAR EL BUNDLE MANIPULADO ================================
synsema run src\verify.syn -- tampered.json
rem exit 1 = REJECTED (correcto). exit 0 = acepto un bundle manipulado = MAL.
if not errorlevel 1 (
  echo.
  echo [FALLO] el bundle manipulado deberia haber sido rechazado
  exit /b 1
)
echo.
echo [ok] el bundle manipulado fue RECHAZADO, como debe ser.
goto :end

:bad
echo.
echo [FALLO] el bundle honesto deberia haber sido verificado
exit /b 1

:end
endlocal
