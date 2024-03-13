$binPath = "C:\mycli\bin\"

Copy-Item .\ir\target\release\ir.exe $binPath -Force
Copy-Item .\zipr\target\release\zipr.exe $binPath -Force

