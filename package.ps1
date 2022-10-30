if (Test-Path .\build) {
    Remove-Item .\build -Recurse
}

mkdir build

Copy-Item C:\gnome\bin\*.dll -Destination .\build\
Copy-Item .\target\release\*.exe -Destination .\build\

Compress-Archive .\build -DestinationPath .\build.zip -Force

Remove-Item .\build -Recurse