install:
	dotnet restore

build:
	dotnet publish -c Release -r win-x64 --self-contained true -o dist

run:
	cd dist && dotnet-service.exe

